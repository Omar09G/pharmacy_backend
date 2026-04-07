-- DB/tables_functions.sql
-- Tablas de reporte con prefijo `vw_t_` y funciones `fn_t_` que refrescan (eliminan e insertan)
-- Las funciones aceptan parámetros opcionales: si se envían NULL o 0 no se aplican en el filtro.

-- 1) Tabla y función: resumen diario de ventas
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_sales_daily_summary (
  day timestamptz,
  sales_count bigint,
  subtotal numeric(14,2),
  tax_total numeric(14,2),
  discount_total numeric(14,2),
  total numeric(14,2),
  total_credit numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_sales_daily_summary(
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL,
  p_customer_id bigint DEFAULT 0,
  p_user_id bigint DEFAULT 0,
  p_status text DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_sales_daily_summary;

  INSERT INTO pharmacy.vw_t_sales_daily_summary(day, sales_count, subtotal, tax_total, discount_total, total, total_credit)
  SELECT
    date_trunc('day', s.date) AS day,
    count(*) AS sales_count,
    sum(s.subtotal) AS subtotal,
    sum(s.tax_total) AS tax_total,
    sum(s.discount_total) AS discount_total,
    sum(s.total) AS total,
    sum(CASE WHEN s.is_credit THEN s.total ELSE 0 END) AS total_credit
  FROM pharmacy.sales s
  WHERE
    -- status: si p_status IS NULL se mantiene el comportamiento original (excluir 'cancelled'),
    -- si p_status NO ES NULL se filtra por ese valor
    ((p_status IS NULL AND s.status IS DISTINCT FROM 'cancelled') OR (p_status IS NOT NULL AND s.status = p_status))
    AND (p_customer_id IS NULL OR p_customer_id = 0 OR s.customer_id = p_customer_id)
    AND (p_user_id IS NULL OR p_user_id = 0 OR s.user_id = p_user_id)
    AND (p_start IS NULL OR s.date >= p_start)
    AND (p_end IS NULL OR s.date <= p_end)
  GROUP BY date_trunc('day', s.date);

  RETURN;
END;
$func$;


-- 2) Tabla y función: ventas con pagos y saldo pendiente (por factura)
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_sales_with_payments (
  id bigint,
  invoice_no text,
  date timestamptz,
  customer_id bigint,
  customer_name text,
  user_id bigint,
  user_name text,
  subtotal numeric(14,2),
  tax_total numeric(14,2),
  discount_total numeric(14,2),
  total numeric(14,2),
  status text,
  is_credit boolean,
  paid_amount numeric(14,2),
  allocated_amount numeric(14,2),
  outstanding numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_sales_with_payments(
  p_sale_id bigint DEFAULT 0,
  p_customer_id bigint DEFAULT 0,
  p_user_id bigint DEFAULT 0,
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL,
  p_is_credit boolean DEFAULT NULL,
  p_status text DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_sales_with_payments;

  INSERT INTO pharmacy.vw_t_sales_with_payments(
    id, invoice_no, date, customer_id, customer_name, user_id, user_name,
    subtotal, tax_total, discount_total, total, status, is_credit,
    paid_amount, allocated_amount, outstanding)
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
    COALESCE(p.total_paid, 0)::numeric(14,2) AS paid_amount,
    COALESCE(a.total_allocated, 0)::numeric(14,2) AS allocated_amount,
    (s.total - COALESCE(p.total_paid, 0) - COALESCE(a.total_allocated, 0))::numeric(14,2) AS outstanding
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
  ) a ON a.credit_invoice_id = s.id
  WHERE
    (p_sale_id IS NULL OR p_sale_id = 0 OR s.id = p_sale_id)
    AND (p_customer_id IS NULL OR p_customer_id = 0 OR s.customer_id = p_customer_id)
    AND (p_user_id IS NULL OR p_user_id = 0 OR s.user_id = p_user_id)
    AND (p_is_credit IS NULL OR s.is_credit = p_is_credit)
    AND (p_status IS NULL OR s.status = p_status)
    AND (p_start IS NULL OR s.date >= p_start)
    AND (p_end IS NULL OR s.date <= p_end);

  RETURN;
END;
$func$;


-- 3) Tabla y función: detalle de líneas de venta
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_sale_items_detail (
  sale_item_id bigint,
  sale_id bigint,
  product_id bigint,
  product_name text,
  lot_id bigint,
  qty numeric(14,4),
  unit_price numeric(14,4),
  discount numeric(14,2),
  tax_amount numeric(14,2),
  line_total numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_sale_items_detail(
  p_sale_id bigint DEFAULT 0,
  p_product_id bigint DEFAULT 0,
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_sale_items_detail;

  INSERT INTO pharmacy.vw_t_sale_items_detail(
    sale_item_id, sale_id, product_id, product_name, lot_id, qty, unit_price, discount, tax_amount, line_total)
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
  LEFT JOIN pharmacy.products p ON p.id = si.product_id
  LEFT JOIN pharmacy.sales s ON s.id = si.sale_id
  WHERE (p_sale_id IS NULL OR p_sale_id = 0 OR si.sale_id = p_sale_id)
    AND (p_product_id IS NULL OR p_product_id = 0 OR si.product_id = p_product_id)
    AND (p_start IS NULL OR s.date >= p_start)
    AND (p_end IS NULL OR s.date <= p_end);

  RETURN;
END;
$func$;


-- 4) Tabla y función: corte diario de caja
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_daily_cash_cut (
  day timestamptz,
  sales_cash numeric(14,2),
  sales_non_cash numeric(14,2),
  cash_entries_in numeric(14,2),
  cash_entries_out numeric(14,2),
  net_cash numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_daily_cash_cut(
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL,
  p_status text DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_daily_cash_cut;

  WITH sales_payments AS (
    SELECT
      date_trunc('day', COALESCE(sp.paid_at, s.date)) AS day,
      sum(CASE WHEN pm.method_type = 'cash' THEN sp.amount ELSE 0 END) AS sales_cash,
      sum(CASE WHEN pm.method_type IS DISTINCT FROM 'cash' OR pm.method_type IS NULL THEN sp.amount ELSE 0 END) AS sales_non_cash
    FROM pharmacy.sale_payments sp
    LEFT JOIN pharmacy.payment_methods pm ON pm.id = sp.method_id
    LEFT JOIN pharmacy.sales s ON s.id = sp.sale_id
    WHERE
      ((p_status IS NULL AND s.status IS DISTINCT FROM 'cancelled') OR (p_status IS NOT NULL AND s.status = p_status))
      AND (p_start IS NULL OR date_trunc('day', COALESCE(sp.paid_at, s.date)) >= date_trunc('day', p_start))
      AND (p_end IS NULL OR date_trunc('day', COALESCE(sp.paid_at, s.date)) <= date_trunc('day', p_end))
    GROUP BY date_trunc('day', COALESCE(sp.paid_at, s.date))
  ),
  cash_entries_day AS (
    SELECT
      date_trunc('day', recorded_at) AS day,
      sum(CASE WHEN entry_type IN ('inflow','sale') THEN amount ELSE 0 END) AS in_amount,
      sum(CASE WHEN entry_type IN ('outflow','expense') THEN amount ELSE 0 END) AS out_amount
    FROM pharmacy.cash_entries
    WHERE (p_start IS NULL OR date_trunc('day', recorded_at) >= date_trunc('day', p_start))
      AND (p_end IS NULL OR date_trunc('day', recorded_at) <= date_trunc('day', p_end))
    GROUP BY date_trunc('day', recorded_at)
  )
  INSERT INTO pharmacy.vw_t_daily_cash_cut(day, sales_cash, sales_non_cash, cash_entries_in, cash_entries_out, net_cash)
  SELECT
    COALESCE(sp.day, ce.day) AS day,
    COALESCE(sp.sales_cash, 0) AS sales_cash,
    COALESCE(sp.sales_non_cash, 0) AS sales_non_cash,
    COALESCE(ce.in_amount, 0) AS cash_entries_in,
    COALESCE(ce.out_amount, 0) AS cash_entries_out,
    (COALESCE(sp.sales_cash, 0) + COALESCE(ce.in_amount, 0) - COALESCE(ce.out_amount, 0)) AS net_cash
  FROM sales_payments sp
  FULL OUTER JOIN cash_entries_day ce ON sp.day = ce.day;

  RETURN;
END;
$func$;


-- 5) Tabla y función: resumen por cliente (facturado, pagado, saldo)
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_customer_account_summary (
  customer_id bigint,
  customer_name text,
  total_invoiced numeric(14,2),
  total_paid numeric(14,2),
  balance numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_customer_account_summary(
  p_customer_id bigint DEFAULT 0,
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_customer_account_summary;

  INSERT INTO pharmacy.vw_t_customer_account_summary(customer_id, customer_name, total_invoiced, total_paid, balance)
  SELECT
    cu.id AS customer_id,
    cu.name AS customer_name,
    COALESCE((SELECT sum(s2.total) FROM pharmacy.sales s2 WHERE s2.customer_id = cu.id
      AND (p_start IS NULL OR s2.date >= p_start)
      AND (p_end IS NULL OR s2.date <= p_end)
      AND s2.status IS DISTINCT FROM 'cancelled'), 0)::numeric(14,2) AS total_invoiced,
    COALESCE((SELECT sum(sp2.amount) FROM pharmacy.sale_payments sp2 JOIN pharmacy.sales s3 ON s3.id = sp2.sale_id
      WHERE s3.customer_id = cu.id
      AND (p_start IS NULL OR sp2.paid_at >= p_start)
      AND (p_end IS NULL OR sp2.paid_at <= p_end)
      AND s3.status IS DISTINCT FROM 'cancelled'), 0)::numeric(14,2) AS total_paid,
    0::numeric(14,2) AS balance
  FROM pharmacy.customers cu
  WHERE (p_customer_id IS NULL OR p_customer_id = 0 OR cu.id = p_customer_id)
  ORDER BY cu.id;

  -- actualizar balance usando los valores calculados
  UPDATE pharmacy.vw_t_customer_account_summary
  SET balance = total_invoiced - total_paid;

  RETURN;
END;
$func$;


-- 6) Tabla y función: detalle de facturas a crédito y aging
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_customer_invoice_aging (
  invoice_id bigint,
  invoice_no text,
  customer_id bigint,
  customer_name text,
  invoice_date timestamptz,
  due_date date,
  paid_amount numeric(14,2),
  outstanding numeric(14,2),
  invoice_status text,
  days_overdue int
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_customer_invoice_aging(
  p_customer_id bigint DEFAULT 0,
  p_as_of date DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
DECLARE
  l_as_of date := COALESCE(p_as_of, now()::date);
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_customer_invoice_aging;

  INSERT INTO pharmacy.vw_t_customer_invoice_aging(
    invoice_id, invoice_no, customer_id, customer_name, invoice_date, due_date, paid_amount, outstanding, invoice_status, days_overdue)
  SELECT
    s.id AS invoice_id,
    s.invoice_no,
    s.customer_id,
    cu.name AS customer_name,
    s.date AS invoice_date,
    (s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date AS due_date,
    COALESCE((SELECT sum(sp.amount) FROM pharmacy.sale_payments sp WHERE sp.sale_id = s.id), 0)::numeric(14,2) AS paid_amount,
    (s.total - COALESCE((SELECT sum(sp.amount) FROM pharmacy.sale_payments sp WHERE sp.sale_id = s.id), 0))::numeric(14,2) AS outstanding,
    CASE
      WHEN (s.total - COALESCE((SELECT sum(sp.amount) FROM pharmacy.sale_payments sp WHERE sp.sale_id = s.id), 0)) <= 0 THEN 'paid'
      WHEN l_as_of > (s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date THEN 'overdue'
      ELSE 'open'
    END AS invoice_status,
    GREATEST(((l_as_of) - ((s.date + (COALESCE(cu.terms_days, 0) * INTERVAL '1 day'))::date))::int, 0) AS days_overdue
  FROM pharmacy.sales s
  LEFT JOIN pharmacy.customers cu ON cu.id = s.customer_id
  WHERE s.is_credit = TRUE
    AND s.status IS DISTINCT FROM 'cancelled'
    AND (p_customer_id IS NULL OR p_customer_id = 0 OR s.customer_id = p_customer_id);

  RETURN;
END;
$func$;


-- 7) Tabla y función: stock por producto (suma de lotes)
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_inventory_stock (
  product_id bigint,
  sku text,
  product_name text,
  qty_on_hand numeric(14,4),
  max_expiry_date date,
  last_movement_at timestamptz
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_inventory_stock(
  p_product_id bigint DEFAULT 0
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_inventory_stock;

  INSERT INTO pharmacy.vw_t_inventory_stock(product_id, sku, product_name, qty_on_hand, max_expiry_date, last_movement_at)
  SELECT
    p.id AS product_id,
    p.sku,
    p.name AS product_name,
    COALESCE((SELECT sum(pl.qty_on_hand) FROM pharmacy.product_lots pl WHERE pl.product_id = p.id), 0)::numeric(14,4) AS qty_on_hand,
    (SELECT max(pl.expiry_date) FROM pharmacy.product_lots pl WHERE pl.product_id = p.id) AS max_expiry_date,
    (SELECT max(im.created_at) FROM pharmacy.inventory_movements im WHERE im.product_id = p.id) AS last_movement_at
  FROM pharmacy.products p
  WHERE (p_product_id IS NULL OR p_product_id = 0 OR p.id = p_product_id)
  ORDER BY p.id;

  RETURN;
END;
$func$;


-- 8) Tabla y función: mejores vendidos (por defecto últimos 30 días)
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_best_sellers_30d (
  product_id bigint,
  sku text,
  product_name text,
  qty_sold numeric(14,4),
  revenue numeric(14,2),
  sales_count bigint
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_best_sellers_30d(
  p_days int DEFAULT 30,
  p_product_id bigint DEFAULT 0,
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
DECLARE
  l_start timestamptz;
  l_end timestamptz;
BEGIN
  l_start := p_start;
  l_end := COALESCE(p_end, now());
  IF l_start IS NULL THEN
    l_start := now() - (p_days || ' days')::interval;
  END IF;

  TRUNCATE TABLE pharmacy.vw_t_best_sellers_30d;

  INSERT INTO pharmacy.vw_t_best_sellers_30d(product_id, sku, product_name, qty_sold, revenue, sales_count)
  SELECT
    p.id AS product_id,
    p.sku,
    p.name AS product_name,
    SUM(si.qty)::numeric(14,4) AS qty_sold,
    SUM(si.line_total)::numeric(14,2) AS revenue,
    COUNT(DISTINCT si.sale_id) AS sales_count
  FROM pharmacy.sale_items si
  JOIN pharmacy.sales s ON s.id = si.sale_id
    AND ((s.status IS DISTINCT FROM 'cancelled'))
    AND s.date >= l_start
    AND s.date <= l_end
  JOIN pharmacy.products p ON p.id = si.product_id
  WHERE (p_product_id IS NULL OR p_product_id = 0 OR p.id = p_product_id)
  GROUP BY p.id, p.sku, p.name
  ORDER BY qty_sold DESC;

  RETURN;
END;
$func$;


-- 9) Tabla y función: balance por cash_journal
CREATE TABLE IF NOT EXISTS pharmacy.vw_t_cash_journal_balance (
  cash_journal_id bigint,
  name text,
  opening_amount numeric(14,2),
  opened_at timestamptz,
  closed_at timestamptz,
  inflow numeric(14,2),
  outflow numeric(14,2),
  balance numeric(14,2)
);

CREATE OR REPLACE FUNCTION pharmacy.fn_t_cash_journal_balance(
  p_cash_journal_id bigint DEFAULT 0,
  p_start timestamptz DEFAULT NULL,
  p_end timestamptz DEFAULT NULL
) RETURNS void LANGUAGE plpgsql AS $func$
BEGIN
  TRUNCATE TABLE pharmacy.vw_t_cash_journal_balance;

  INSERT INTO pharmacy.vw_t_cash_journal_balance(
    cash_journal_id, name, opening_amount, opened_at, closed_at, inflow, outflow, balance)
  SELECT
    cj.id AS cash_journal_id,
    cj.name,
    cj.opening_amount,
    cj.opened_at,
    cj.closed_at,
    COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('inflow','sale') AND (p_start IS NULL OR ce.recorded_at >= p_start) AND (p_end IS NULL OR ce.recorded_at <= p_end)), 0)::numeric(14,2) AS inflow,
    COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('outflow','expense') AND (p_start IS NULL OR ce.recorded_at >= p_start) AND (p_end IS NULL OR ce.recorded_at <= p_end)), 0)::numeric(14,2) AS outflow,
    (cj.opening_amount + COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('inflow','sale') AND (p_start IS NULL OR ce.recorded_at >= p_start) AND (p_end IS NULL OR ce.recorded_at <= p_end)), 0) - COALESCE(SUM(ce.amount) FILTER (WHERE ce.entry_type IN ('outflow','expense') AND (p_start IS NULL OR ce.recorded_at >= p_start) AND (p_end IS NULL OR ce.recorded_at <= p_end)), 0))::numeric(14,2) AS balance
  FROM pharmacy.cash_journals cj
  LEFT JOIN pharmacy.cash_entries ce ON ce.related_type = 'cash_journal' AND ce.related_id = cj.id
  WHERE (p_cash_journal_id IS NULL OR p_cash_journal_id = 0 OR cj.id = p_cash_journal_id)
  GROUP BY cj.id, cj.name, cj.opening_amount, cj.opened_at, cj.closed_at
  ORDER BY cj.id;

  RETURN;
END;
$func$;

-- Fin de DB/tables_functions.sql
