-- DB/views.sql
-- Vistas para reportes y validaciones del sistema
-- Incluye: resúmenes de ventas, pagos, cortes diarios, cajas, cuentas de clientes, stock y mejores vendedores

-- 1) Resumen diario de ventas
CREATE OR REPLACE VIEW pharmacy.vw_sales_daily_summary AS
SELECT
  date_trunc('day', s.date) AS day,
  count(*) AS sales_count,
  sum(s.subtotal) AS subtotal,
  sum(s.tax_total) AS tax_total,
  sum(s.discount_total) AS discount_total,
  sum(s.total) AS total,
  sum(CASE WHEN s.is_credit THEN s.total ELSE 0 END) AS total_credit
FROM pharmacy.sales s
WHERE s.status IS DISTINCT FROM 'cancelled'
GROUP BY date_trunc('day', s.date);


-- 2) Ventas con totales de pagos y saldo pendiente (por factura)
CREATE OR REPLACE VIEW pharmacy.vw_sales_with_payments AS
SELECT
  s.id,
  s.invoice_no,
  s.date,
  s.customer_id,
  c.name AS customer_name,
  s.user_id,
  u.full_name AS user_name,
  s.subtotal,
  s.tax_total,
  s.discount_total,
  s.total,
  s.status,
  s.is_credit,
  COALESCE(p.total_paid, 0) AS paid_amount,
  COALESCE(a.total_allocated, 0) AS allocated_amount,
  (s.total - COALESCE(p.total_paid, 0) - COALESCE(a.total_allocated, 0)) AS outstanding
FROM pharmacy.sales s
LEFT JOIN pharmacy.customers c ON c.id = s.customer_id
LEFT JOIN pharmacy.users u ON u.id = s.user_id
LEFT JOIN (
  SELECT sale_id, sum(amount) AS total_paid
  FROM pharmacy.sale_payments
  GROUP BY sale_id
) p ON p.sale_id = s.id
LEFT JOIN (
  SELECT credit_invoice_id, sum(amount) AS total_allocated
  FROM pharmacy.sale_payment_allocations
  GROUP BY credit_invoice_id
) a ON a.credit_invoice_id = s.id;


-- 3) Detalle de líneas de venta (por item)
CREATE OR REPLACE VIEW pharmacy.vw_sale_items_detail AS
SELECT
  si.id AS sale_item_id,
  si.sale_id,
  si.product_id,
  p.name AS product_name,
  si.lot_id,
  si.qty,
  si.unit_price,
  si.discount,
  si.tax_amount,
  si.line_total
FROM pharmacy.sale_items si
LEFT JOIN pharmacy.products p ON p.id = si.product_id;


-- 4) Corte diario de caja (resumen por día: ventas en efectivo y movimientos de caja)
CREATE OR REPLACE VIEW pharmacy.vw_daily_cash_cut AS
WITH sales_payments AS (
  SELECT
    date_trunc('day', COALESCE(sp.paid_at, s.date)) AS day,
    sum(CASE WHEN pm.method_type = 'cash' THEN sp.amount ELSE 0 END) AS sales_cash,
    sum(CASE WHEN pm.method_type IS DISTINCT FROM 'cash' OR pm.method_type IS NULL THEN sp.amount ELSE 0 END) AS sales_non_cash
  FROM pharmacy.sale_payments sp
  LEFT JOIN pharmacy.payment_methods pm ON pm.id = sp.method_id
  LEFT JOIN pharmacy.sales s ON s.id = sp.sale_id
  WHERE s.status IS DISTINCT FROM 'cancelled'
  GROUP BY date_trunc('day', COALESCE(sp.paid_at, s.date))
),
cash_entries_day AS (
  SELECT
    date_trunc('day', recorded_at) AS day,
    sum(CASE WHEN entry_type IN ('inflow','sale') THEN amount ELSE 0 END) AS in_amount,
    sum(CASE WHEN entry_type IN ('outflow','expense') THEN amount ELSE 0 END) AS out_amount
  FROM pharmacy.cash_entries
  GROUP BY date_trunc('day', recorded_at)
)
SELECT
  COALESCE(sp.day, ce.day) AS day,
  COALESCE(sp.sales_cash, 0) AS sales_cash,
  COALESCE(sp.sales_non_cash, 0) AS sales_non_cash,
  COALESCE(ce.in_amount, 0) AS cash_entries_in,
  COALESCE(ce.out_amount, 0) AS cash_entries_out,
  (COALESCE(sp.sales_cash, 0) + COALESCE(ce.in_amount, 0) - COALESCE(ce.out_amount, 0)) AS net_cash
FROM sales_payments sp
FULL OUTER JOIN cash_entries_day ce ON sp.day = ce.day;


-- 5) Resumen por cliente: facturado, pagado y saldo
CREATE OR REPLACE VIEW pharmacy.vw_customer_account_summary AS
SELECT
  cu.id AS customer_id,
  cu.name AS customer_name,
  COALESCE(SUM(s.total) FILTER (WHERE s.status IS DISTINCT FROM 'cancelled'), 0) AS total_invoiced,
  COALESCE(SUM(sp.amount), 0) AS total_paid,
  COALESCE(SUM(s.total) FILTER (WHERE s.status IS DISTINCT FROM 'cancelled'), 0) - COALESCE(SUM(sp.amount), 0) AS balance
FROM pharmacy.customers cu
LEFT JOIN pharmacy.sales s ON s.customer_id = cu.id
LEFT JOIN pharmacy.sale_payments sp ON sp.sale_id = s.id
GROUP BY cu.id, cu.name;


-- 6) Detalle de facturas a crédito y antigüedad (aging) por factura
CREATE OR REPLACE VIEW pharmacy.vw_customer_invoice_aging AS
SELECT
  s.id AS invoice_id,
  s.invoice_no,
  s.customer_id,
  cu.name AS customer_name,
  s.date AS invoice_date,
  (s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date AS due_date,
  COALESCE(sp.total_paid, 0) AS paid_amount,
  (s.total - COALESCE(sp.total_paid, 0)) AS outstanding,
  CASE
    WHEN (s.total - COALESCE(sp.total_paid, 0)) <= 0 THEN 'paid'
    WHEN now()::date > (s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date THEN 'overdue'
    ELSE 'open'
  END AS invoice_status,
  GREATEST(((now()::date) - ((s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date))::int, 0) AS days_overdue
FROM pharmacy.sales s
LEFT JOIN pharmacy.customers cu ON cu.id = s.customer_id
LEFT JOIN (
  SELECT sale_id, sum(amount) AS total_paid
  FROM pharmacy.sale_payments
  GROUP BY sale_id
) sp ON sp.sale_id = s.id
WHERE s.is_credit = TRUE AND s.status IS DISTINCT FROM 'cancelled';


-- 7) Stock por producto (suma de lotes)
CREATE OR REPLACE VIEW pharmacy.vw_inventory_stock AS
SELECT
  p.id AS product_id,
  p.sku,
  p.name AS product_name,
  COALESCE(SUM(pl.qty_on_hand), 0) AS qty_on_hand,
  MAX(pl.expiry_date) AS max_expiry_date,
  MAX(im.created_at) AS last_movement_at
FROM pharmacy.products p
LEFT JOIN pharmacy.product_lots pl ON pl.product_id = p.id
LEFT JOIN pharmacy.inventory_movements im ON im.product_id = p.id
GROUP BY p.id, p.sku, p.name;


-- 8) Mejores vendedores (últimos 30 días)
CREATE OR REPLACE VIEW pharmacy.vw_best_sellers_30d AS
SELECT
  p.id AS product_id,
  p.sku,
  p.name AS product_name,
  SUM(si.qty) AS qty_sold,
  SUM(si.line_total) AS revenue,
  COUNT(DISTINCT si.sale_id) AS sales_count
FROM pharmacy.sale_items si
JOIN pharmacy.sales s ON s.id = si.sale_id AND s.status IS DISTINCT FROM 'cancelled' AND s.date >= (now() - INTERVAL '30 days')
JOIN pharmacy.products p ON p.id = si.product_id
GROUP BY p.id, p.sku, p.name;


-- 9) Balance por caja (cash_journal) — apertura + entradas/salidas referenciadas
CREATE OR REPLACE VIEW pharmacy.vw_cash_journal_balance AS
SELECT
  cj.id AS cash_journal_id,
  cj.name,
  cj.opening_amount,
  cj.opened_at,
  cj.closed_at,
  COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('inflow','sale')), 0) AS inflow,
  COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('outflow','expense')), 0) AS outflow,
  (cj.opening_amount + COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('inflow','sale')), 0) - COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('outflow','expense')), 0)) AS balance
FROM pharmacy.cash_journals cj
LEFT JOIN pharmacy.cash_entries ce ON ce.related_type = 'cash_journal' AND ce.related_id = cj.id
GROUP BY cj.id, cj.name, cj.opening_amount, cj.opened_at, cj.closed_at;


-- Nota: estas vistas están pensadas para uso de consulta y reportes. Para cortar día exacto,
-- filtrar por la columna `day` (en vw_daily_cash_cut) o por `date`/`paid_at` en las vistas de ventas/pagos.
