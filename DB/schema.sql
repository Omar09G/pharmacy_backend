-- DROP SCHEMA pharmacy;

CREATE SCHEMA pharmacy AUTHORIZATION postgres;

-- DROP SEQUENCE pharmacy.seq_audit_log;

CREATE SEQUENCE pharmacy.seq_audit_log
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_audit_log OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_audit_log TO postgres;

-- DROP SEQUENCE pharmacy.seq_cash_entries;

CREATE SEQUENCE pharmacy.seq_cash_entries
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_cash_entries OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_cash_entries TO postgres;

-- DROP SEQUENCE pharmacy.seq_cash_journals;

CREATE SEQUENCE pharmacy.seq_cash_journals
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_cash_journals OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_cash_journals TO postgres;

-- DROP SEQUENCE pharmacy.seq_categories;

CREATE SEQUENCE pharmacy.seq_categories
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_categories OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_categories TO postgres;

-- DROP SEQUENCE pharmacy.seq_config_audit;

CREATE SEQUENCE pharmacy.seq_config_audit
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_config_audit OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_config_audit TO postgres;

-- DROP SEQUENCE pharmacy.seq_customer_credit_accounts;

CREATE SEQUENCE pharmacy.seq_customer_credit_accounts
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_customer_credit_accounts OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_customer_credit_accounts TO postgres;

-- DROP SEQUENCE pharmacy.seq_customers;

CREATE SEQUENCE pharmacy.seq_customers
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_customers OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_customers TO postgres;

-- DROP SEQUENCE pharmacy.seq_discounts;

CREATE SEQUENCE pharmacy.seq_discounts
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_discounts OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_discounts TO postgres;

-- DROP SEQUENCE pharmacy.seq_inventory_locations;

CREATE SEQUENCE pharmacy.seq_inventory_locations
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_inventory_locations OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_inventory_locations TO postgres;

-- DROP SEQUENCE pharmacy.seq_inventory_movements;

CREATE SEQUENCE pharmacy.seq_inventory_movements
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_inventory_movements OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_inventory_movements TO postgres;

-- DROP SEQUENCE pharmacy.seq_payment_methods;

CREATE SEQUENCE pharmacy.seq_payment_methods
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_payment_methods OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_payment_methods TO postgres;

-- DROP SEQUENCE pharmacy.seq_permissions;

CREATE SEQUENCE pharmacy.seq_permissions
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_permissions OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_permissions TO postgres;

-- DROP SEQUENCE pharmacy.seq_product_barcodes;

CREATE SEQUENCE pharmacy.seq_product_barcodes
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_product_barcodes OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_product_barcodes TO postgres;

-- DROP SEQUENCE pharmacy.seq_product_lots;

CREATE SEQUENCE pharmacy.seq_product_lots
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_product_lots OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_product_lots TO postgres;

-- DROP SEQUENCE pharmacy.seq_product_prices;

CREATE SEQUENCE pharmacy.seq_product_prices
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_product_prices OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_product_prices TO postgres;

-- DROP SEQUENCE pharmacy.seq_products;

CREATE SEQUENCE pharmacy.seq_products
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_products OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_products TO postgres;

-- DROP SEQUENCE pharmacy.seq_purchase_items;

CREATE SEQUENCE pharmacy.seq_purchase_items
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_purchase_items OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_purchase_items TO postgres;

-- DROP SEQUENCE pharmacy.seq_purchase_payments;

CREATE SEQUENCE pharmacy.seq_purchase_payments
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_purchase_payments OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_purchase_payments TO postgres;

-- DROP SEQUENCE pharmacy.seq_purchases;

CREATE SEQUENCE pharmacy.seq_purchases
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_purchases OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_purchases TO postgres;

-- DROP SEQUENCE pharmacy.seq_roles;

CREATE SEQUENCE pharmacy.seq_roles
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_roles OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_roles TO postgres;

-- DROP SEQUENCE pharmacy.seq_sale_items;

CREATE SEQUENCE pharmacy.seq_sale_items
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_sale_items OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_sale_items TO postgres;

-- DROP SEQUENCE pharmacy.seq_sale_payment_allocations;

CREATE SEQUENCE pharmacy.seq_sale_payment_allocations
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_sale_payment_allocations OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_sale_payment_allocations TO postgres;

-- DROP SEQUENCE pharmacy.seq_sale_payments;

CREATE SEQUENCE pharmacy.seq_sale_payments
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_sale_payments OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_sale_payments TO postgres;

-- DROP SEQUENCE pharmacy.seq_sales;

CREATE SEQUENCE pharmacy.seq_sales
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_sales OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_sales TO postgres;

-- DROP SEQUENCE pharmacy.seq_suppliers;

CREATE SEQUENCE pharmacy.seq_suppliers
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_suppliers OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_suppliers TO postgres;

-- DROP SEQUENCE pharmacy.seq_tax_profiles;

CREATE SEQUENCE pharmacy.seq_tax_profiles
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_tax_profiles OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_tax_profiles TO postgres;

-- DROP SEQUENCE pharmacy.seq_units;

CREATE SEQUENCE pharmacy.seq_units
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_units OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_units TO postgres;

-- DROP SEQUENCE pharmacy.seq_users;

CREATE SEQUENCE pharmacy.seq_users
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE pharmacy.seq_users OWNER TO postgres;
GRANT ALL ON SEQUENCE pharmacy.seq_users TO postgres;
-- pharmacy.commission_parameters definition

-- Drop table

-- DROP TABLE pharmacy.commission_parameters;

CREATE TABLE pharmacy.commission_parameters ( id int8 DEFAULT nextval('pharmacy.seq_config_audit'::regclass) NOT NULL, "name" text NOT NULL, code text NULL, applies_to text NULL, percentage numeric(6, 4) DEFAULT 0.0 NOT NULL, active bool DEFAULT true NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, description text NULL, CONSTRAINT commission_parameters_code_key UNIQUE (code), CONSTRAINT commission_parameters_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.commission_parameters OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.commission_parameters TO postgres;


-- pharmacy.config_parameters definition

-- Drop table

-- DROP TABLE pharmacy.config_parameters;

CREATE TABLE pharmacy.config_parameters ( "key" text NOT NULL, value text NULL, description text NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT config_parameters_pkey PRIMARY KEY (key));

-- Permissions

ALTER TABLE pharmacy.config_parameters OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.config_parameters TO postgres;


-- pharmacy.customers definition

-- Drop table

-- DROP TABLE pharmacy.customers;

CREATE TABLE pharmacy.customers ( id int8 DEFAULT nextval('pharmacy.seq_customers'::regclass) NOT NULL, "name" text NOT NULL, document_id text NULL, phone text NULL, email text NULL, billing_address text NULL, credit_limit numeric(14, 2) DEFAULT 0.0 NULL, terms_days int4 DEFAULT 0 NULL, status text DEFAULT 'active'::text NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT customers_credit_limit_check CHECK ((credit_limit >= (0)::numeric)), CONSTRAINT customers_pkey PRIMARY KEY (id));
CREATE INDEX idx_customers_name ON pharmacy.customers USING gin (to_tsvector('spanish'::regconfig, name));

-- Permissions

ALTER TABLE pharmacy.customers OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.customers TO postgres;


-- pharmacy.inventory_locations definition

-- Drop table

-- DROP TABLE pharmacy.inventory_locations;

CREATE TABLE pharmacy.inventory_locations ( id int8 DEFAULT nextval('pharmacy.seq_inventory_locations'::regclass) NOT NULL, "name" text NOT NULL, "type" text NULL, description text NULL, CONSTRAINT inventory_locations_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.inventory_locations OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.inventory_locations TO postgres;


-- pharmacy.payment_methods definition

-- Drop table

-- DROP TABLE pharmacy.payment_methods;

CREATE TABLE pharmacy.payment_methods ( id int8 DEFAULT nextval('pharmacy.seq_payment_methods'::regclass) NOT NULL, "name" text NOT NULL, method_type text NULL, active bool DEFAULT true NOT NULL, CONSTRAINT payment_methods_name_key UNIQUE (name), CONSTRAINT payment_methods_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.payment_methods OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.payment_methods TO postgres;


-- pharmacy.permissions definition

-- Drop table

-- DROP TABLE pharmacy.permissions;

CREATE TABLE pharmacy.permissions ( id int8 DEFAULT nextval('pharmacy.seq_permissions'::regclass) NOT NULL, "name" text NOT NULL, description text NULL, CONSTRAINT permissions_name_key UNIQUE (name), CONSTRAINT permissions_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.permissions OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.permissions TO postgres;


-- pharmacy.roles definition

-- Drop table

-- DROP TABLE pharmacy.roles;

CREATE TABLE pharmacy.roles ( id int8 DEFAULT nextval('pharmacy.seq_roles'::regclass) NOT NULL, "name" text NOT NULL, description text NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT roles_name_key UNIQUE (name), CONSTRAINT roles_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.roles OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.roles TO postgres;


-- pharmacy.suppliers definition

-- Drop table

-- DROP TABLE pharmacy.suppliers;

CREATE TABLE pharmacy.suppliers ( id int8 DEFAULT nextval('pharmacy.seq_suppliers'::regclass) NOT NULL, "name" text NOT NULL, tax_id text NULL, contact_person text NULL, phone text NULL, email text NULL, address text NULL, notes text NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT suppliers_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.suppliers OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.suppliers TO postgres;


-- pharmacy.t_accounts_payable definition

-- Drop table

-- DROP TABLE pharmacy.t_accounts_payable;

CREATE TABLE pharmacy.t_accounts_payable ( purchase_id int8 NULL, invoice_no text NULL, supplier_id int8 NULL, supplier_name text NULL, invoice_date timestamptz NULL, invoice_total numeric(14, 2) NULL, paid_amount numeric NULL, outstanding_amount numeric NULL);

-- Permissions

ALTER TABLE pharmacy.t_accounts_payable OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.t_accounts_payable TO postgres;


-- pharmacy.tax_profiles definition

-- Drop table

-- DROP TABLE pharmacy.tax_profiles;

CREATE TABLE pharmacy.tax_profiles ( id int8 DEFAULT nextval('pharmacy.seq_tax_profiles'::regclass) NOT NULL, "name" text NOT NULL, rate numeric(6, 4) DEFAULT 0.0 NOT NULL, is_inclusive bool DEFAULT false NOT NULL, description text NULL, CONSTRAINT tax_profiles_name_key UNIQUE (name), CONSTRAINT tax_profiles_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.tax_profiles OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.tax_profiles TO postgres;


-- pharmacy.tax_rates definition

-- Drop table

-- DROP TABLE pharmacy.tax_rates;

CREATE TABLE pharmacy.tax_rates ( id int8 DEFAULT nextval('pharmacy.seq_tax_profiles'::regclass) NOT NULL, code text NOT NULL, "name" text NOT NULL, rate numeric(6, 4) DEFAULT 0.0 NOT NULL, is_inclusive bool DEFAULT false NOT NULL, applies_to text NULL, active bool DEFAULT true NOT NULL, description text NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT tax_rates_code_key UNIQUE (code), CONSTRAINT tax_rates_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.tax_rates OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.tax_rates TO postgres;


-- pharmacy.units definition

-- Drop table

-- DROP TABLE pharmacy.units;

CREATE TABLE pharmacy.units ( id int8 DEFAULT nextval('pharmacy.seq_units'::regclass) NOT NULL, code text NOT NULL, "name" text NOT NULL, "precision" int4 DEFAULT 0 NOT NULL, CONSTRAINT units_code_key UNIQUE (code), CONSTRAINT units_pkey PRIMARY KEY (id));

-- Permissions

ALTER TABLE pharmacy.units OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.units TO postgres;


-- pharmacy.categories definition

-- Drop table

-- DROP TABLE pharmacy.categories;

CREATE TABLE pharmacy.categories ( id int8 DEFAULT nextval('pharmacy.seq_categories'::regclass) NOT NULL, "name" text NOT NULL, parent_id int8 NULL, description text NULL, CONSTRAINT categories_pkey PRIMARY KEY (id), CONSTRAINT categories_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES pharmacy.categories(id) ON DELETE SET NULL);

-- Permissions

ALTER TABLE pharmacy.categories OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.categories TO postgres;


-- pharmacy.customer_credit_accounts definition

-- Drop table

-- DROP TABLE pharmacy.customer_credit_accounts;

CREATE TABLE pharmacy.customer_credit_accounts ( id int8 DEFAULT nextval('pharmacy.seq_customer_credit_accounts'::regclass) NOT NULL, customer_id int8 NOT NULL, balance numeric(14, 2) DEFAULT 0.0 NOT NULL, limit_amount numeric(14, 2) DEFAULT 0.0 NULL, last_overdue_date date NULL, CONSTRAINT customer_credit_accounts_pkey PRIMARY KEY (id), CONSTRAINT customer_credit_accounts_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES pharmacy.customers(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.customer_credit_accounts OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.customer_credit_accounts TO postgres;


-- pharmacy.products definition

-- Drop table

-- DROP TABLE pharmacy.products;

CREATE TABLE pharmacy.products ( id int8 DEFAULT nextval('pharmacy.seq_products'::regclass) NOT NULL, sku text NULL, "name" text NOT NULL, description text NULL, brand text NULL, category_id int8 NULL, unit_id int8 NULL, is_sellable bool DEFAULT true NOT NULL, track_batches bool DEFAULT false NOT NULL, tax_profile_id int8 NULL, default_cost numeric(14, 2) DEFAULT 0.0 NULL, purchase_price numeric(14, 2) DEFAULT 0.0 NULL, wholesale_price numeric(14, 2) DEFAULT 0.0 NULL, sale_price numeric(14, 2) DEFAULT 0.0 NULL, default_price numeric(14, 2) DEFAULT 0.0 NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz NULL, deleted_at timestamptz NULL, CONSTRAINT products_default_cost_check CHECK ((default_cost >= (0)::numeric)), CONSTRAINT products_default_price_check CHECK ((default_price >= (0)::numeric)), CONSTRAINT products_pkey PRIMARY KEY (id), CONSTRAINT products_purchase_price_check CHECK ((purchase_price >= (0)::numeric)), CONSTRAINT products_sale_price_check CHECK ((sale_price >= (0)::numeric)), CONSTRAINT products_sku_key UNIQUE (sku), CONSTRAINT products_wholesale_price_check CHECK ((wholesale_price >= (0)::numeric)), CONSTRAINT products_category_id_fkey FOREIGN KEY (category_id) REFERENCES pharmacy.categories(id) ON DELETE SET NULL, CONSTRAINT products_tax_profile_id_fkey FOREIGN KEY (tax_profile_id) REFERENCES pharmacy.tax_profiles(id) ON DELETE SET NULL, CONSTRAINT products_unit_id_fkey FOREIGN KEY (unit_id) REFERENCES pharmacy.units(id) ON DELETE SET NULL);
CREATE INDEX idx_products_name ON pharmacy.products USING gin (to_tsvector('spanish'::regconfig, name));
CREATE INDEX idx_products_sku ON pharmacy.products USING btree (sku);

-- Permissions

ALTER TABLE pharmacy.products OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.products TO postgres;


-- pharmacy.role_permissions definition

-- Drop table

-- DROP TABLE pharmacy.role_permissions;

CREATE TABLE pharmacy.role_permissions ( role_id int8 NOT NULL, permission_id int8 NOT NULL, CONSTRAINT role_permissions_pkey PRIMARY KEY (role_id, permission_id), CONSTRAINT role_permissions_permission_id_fkey FOREIGN KEY (permission_id) REFERENCES pharmacy.permissions(id) ON DELETE CASCADE, CONSTRAINT role_permissions_role_id_fkey FOREIGN KEY (role_id) REFERENCES pharmacy.roles(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.role_permissions OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.role_permissions TO postgres;


-- pharmacy.users definition

-- Drop table

-- DROP TABLE pharmacy.users;

CREATE TABLE pharmacy.users ( id int8 DEFAULT nextval('pharmacy.seq_users'::regclass) NOT NULL, username text NOT NULL, password_hash text NOT NULL, full_name text NULL, email text NULL, phone text NULL, status text DEFAULT 'active'::text NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, created_by int8 NULL, updated_at timestamptz NULL, updated_by int8 NULL, deleted_at timestamptz NULL, CONSTRAINT users_email_key UNIQUE (email), CONSTRAINT users_pkey PRIMARY KEY (id), CONSTRAINT users_username_key UNIQUE (username), CONSTRAINT users_created_by_fkey FOREIGN KEY (created_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL, CONSTRAINT users_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL);

-- Permissions

ALTER TABLE pharmacy.users OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.users TO postgres;


-- pharmacy.audit_log definition

-- Drop table

-- DROP TABLE pharmacy.audit_log;

CREATE TABLE pharmacy.audit_log ( id int8 DEFAULT nextval('pharmacy.seq_audit_log'::regclass) NOT NULL, entity_type text NOT NULL, table_name text NULL, entity_id int8 NULL, "action" text NOT NULL, changed_by int8 NULL, changed_at timestamptz DEFAULT now() NOT NULL, change_data jsonb NULL, CONSTRAINT audit_log_pkey PRIMARY KEY (id), CONSTRAINT audit_log_changed_by_fkey FOREIGN KEY (changed_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL);

-- Permissions

ALTER TABLE pharmacy.audit_log OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.audit_log TO postgres;


-- pharmacy.cash_entries definition

-- Drop table

-- DROP TABLE pharmacy.cash_entries;

CREATE TABLE pharmacy.cash_entries ( id int8 DEFAULT nextval('pharmacy.seq_cash_entries'::regclass) NOT NULL, "name" text NOT NULL, entry_type text NOT NULL, amount numeric(14, 2) NOT NULL, method_id int8 NULL, related_type text NULL, related_id int8 NULL, description text NULL, recorded_at timestamptz DEFAULT now() NOT NULL, recorded_by int8 NULL, CONSTRAINT cash_entries_amount_check CHECK ((amount >= (0)::numeric)), CONSTRAINT cash_entries_entry_type_check CHECK ((entry_type = ANY (ARRAY['inflow'::text, 'outflow'::text, 'adjustment'::text, 'sale'::text, 'purchase'::text, 'expense'::text, 'other'::text]))), CONSTRAINT cash_entries_pkey PRIMARY KEY (id), CONSTRAINT cash_entries_method_id_fkey FOREIGN KEY (method_id) REFERENCES pharmacy.payment_methods(id), CONSTRAINT cash_entries_recorded_by_fkey FOREIGN KEY (recorded_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL);
CREATE INDEX idx_cash_entries_recorded_at ON pharmacy.cash_entries USING btree (recorded_at);

-- Permissions

ALTER TABLE pharmacy.cash_entries OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.cash_entries TO postgres;


-- pharmacy.cash_journals definition

-- Drop table

-- DROP TABLE pharmacy.cash_journals;

CREATE TABLE pharmacy.cash_journals ( id int8 DEFAULT nextval('pharmacy.seq_cash_journals'::regclass) NOT NULL, "name" text NOT NULL, description text NULL, opening_amount numeric(14, 2) DEFAULT 0.0 NOT NULL, opened_at timestamptz DEFAULT now() NOT NULL, closed_at timestamptz NULL, opened_by int8 NULL, closed_by int8 NULL, status text DEFAULT 'open'::text NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT cash_journals_pkey PRIMARY KEY (id), CONSTRAINT cash_journals_closed_by_fkey FOREIGN KEY (closed_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL, CONSTRAINT cash_journals_opened_by_fkey FOREIGN KEY (opened_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL);
CREATE INDEX idx_cash_journals_opened_at ON pharmacy.cash_journals USING btree (opened_at);

-- Permissions

ALTER TABLE pharmacy.cash_journals OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.cash_journals TO postgres;


-- pharmacy.discounts definition

-- Drop table

-- DROP TABLE pharmacy.discounts;

CREATE TABLE pharmacy.discounts ( id int8 DEFAULT nextval('pharmacy.seq_discounts'::regclass) NOT NULL, code text NULL, "name" text NOT NULL, description text NULL, discount_type text NOT NULL, value numeric(14, 4) NOT NULL, applies_to text DEFAULT 'all'::text NOT NULL, product_id int8 NULL, category_id int8 NULL, customer_id int8 NULL, min_qty numeric(14, 4) DEFAULT 0 NULL, max_uses int8 NULL, priority int4 DEFAULT 100 NULL, start_at timestamptz NULL, end_at timestamptz NULL, active bool DEFAULT true NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, created_by int8 NULL, CONSTRAINT discounts_code_key UNIQUE (code), CONSTRAINT discounts_discount_type_check CHECK ((discount_type = ANY (ARRAY['percentage'::text, 'fixed'::text]))), CONSTRAINT discounts_pkey PRIMARY KEY (id), CONSTRAINT discounts_value_check CHECK ((value >= (0)::numeric)), CONSTRAINT discounts_category_id_fkey FOREIGN KEY (category_id) REFERENCES pharmacy.categories(id) ON DELETE SET NULL, CONSTRAINT discounts_created_by_fkey FOREIGN KEY (created_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL, CONSTRAINT discounts_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES pharmacy.customers(id) ON DELETE SET NULL, CONSTRAINT discounts_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.discounts OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.discounts TO postgres;


-- pharmacy.product_barcodes definition

-- Drop table

-- DROP TABLE pharmacy.product_barcodes;

CREATE TABLE pharmacy.product_barcodes ( id int8 DEFAULT nextval('pharmacy.seq_product_barcodes'::regclass) NOT NULL, product_id int8 NOT NULL, barcode text NOT NULL, barcode_type text NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT product_barcodes_barcode_key UNIQUE (barcode), CONSTRAINT product_barcodes_pkey PRIMARY KEY (id), CONSTRAINT product_barcodes_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.product_barcodes OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.product_barcodes TO postgres;


-- pharmacy.product_prices definition

-- Drop table

-- DROP TABLE pharmacy.product_prices;

CREATE TABLE pharmacy.product_prices ( id int8 DEFAULT nextval('pharmacy.seq_product_prices'::regclass) NOT NULL, product_id int8 NOT NULL, price_type text NOT NULL, price numeric(14, 2) NOT NULL, starts_at timestamptz NULL, ends_at timestamptz NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT product_prices_pkey PRIMARY KEY (id), CONSTRAINT product_prices_price_check CHECK ((price >= (0)::numeric)), CONSTRAINT product_prices_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.product_prices OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.product_prices TO postgres;


-- pharmacy.purchases definition

-- Drop table

-- DROP TABLE pharmacy.purchases;

CREATE TABLE pharmacy.purchases ( id int8 DEFAULT nextval('pharmacy.seq_purchases'::regclass) NOT NULL, supplier_id int8 NULL, invoice_no text NULL, "date" timestamptz DEFAULT now() NOT NULL, subtotal numeric(14, 2) DEFAULT 0.0 NOT NULL, tax_total numeric(14, 2) DEFAULT 0.0 NOT NULL, total numeric(14, 2) DEFAULT 0.0 NOT NULL, status text DEFAULT 'draft'::text NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, created_by int8 NULL, CONSTRAINT purchases_pkey PRIMARY KEY (id), CONSTRAINT purchases_subtotal_check CHECK ((subtotal >= (0)::numeric)), CONSTRAINT purchases_tax_total_check CHECK ((tax_total >= (0)::numeric)), CONSTRAINT purchases_total_check CHECK ((total >= (0)::numeric)), CONSTRAINT purchases_created_by_fkey FOREIGN KEY (created_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL, CONSTRAINT purchases_supplier_id_fkey FOREIGN KEY (supplier_id) REFERENCES pharmacy.suppliers(id) ON DELETE SET NULL);
CREATE INDEX idx_purchases_date ON pharmacy.purchases USING btree (date);

-- Permissions

ALTER TABLE pharmacy.purchases OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.purchases TO postgres;


-- pharmacy.sales definition

-- Drop table

-- DROP TABLE pharmacy.sales;

CREATE TABLE pharmacy.sales ( id int8 DEFAULT nextval('pharmacy.seq_sales'::regclass) NOT NULL, customer_id int8 NULL, user_id int8 NULL, invoice_no text NULL, "date" timestamptz DEFAULT now() NOT NULL, subtotal numeric(14, 2) DEFAULT 0.0 NOT NULL, tax_total numeric(14, 2) DEFAULT 0.0 NOT NULL, discount_total numeric(14, 2) DEFAULT 0.0 NOT NULL, total numeric(14, 2) DEFAULT 0.0 NOT NULL, status text DEFAULT 'draft'::text NOT NULL, is_credit bool DEFAULT false NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT sales_pkey PRIMARY KEY (id), CONSTRAINT sales_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES pharmacy.customers(id) ON DELETE SET NULL, CONSTRAINT sales_user_id_fkey FOREIGN KEY (user_id) REFERENCES pharmacy.users(id) ON DELETE SET NULL);
CREATE INDEX idx_sales_customer_status ON pharmacy.sales USING btree (customer_id, status) INCLUDE (total, date);
CREATE INDEX idx_sales_date ON pharmacy.sales USING btree (date);
CREATE INDEX idx_sales_status ON pharmacy.sales USING btree (status);

-- Table Triggers

create trigger trg_revert_stock_on_sale_cancel after
update
    on
    pharmacy.sales for each row
    when (((old.status is distinct
from
    new.status)
    and (new.status = 'cancelled'::text))) execute function pharmacy.fn_revert_stock_on_sale_cancel();

-- Permissions

ALTER TABLE pharmacy.sales OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.sales TO postgres;


-- pharmacy.user_roles definition

-- Drop table

-- DROP TABLE pharmacy.user_roles;

CREATE TABLE pharmacy.user_roles ( user_id int8 NOT NULL, role_id int8 NOT NULL, CONSTRAINT user_roles_pkey PRIMARY KEY (user_id, role_id), CONSTRAINT user_roles_role_id_fkey FOREIGN KEY (role_id) REFERENCES pharmacy.roles(id) ON DELETE CASCADE, CONSTRAINT user_roles_user_id_fkey FOREIGN KEY (user_id) REFERENCES pharmacy.users(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.user_roles OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.user_roles TO postgres;


-- pharmacy.product_lots definition

-- Drop table

-- DROP TABLE pharmacy.product_lots;

CREATE TABLE pharmacy.product_lots ( id int8 DEFAULT nextval('pharmacy.seq_product_lots'::regclass) NOT NULL, product_id int8 NOT NULL, lot_number text NULL, qty_on_hand numeric(14, 4) DEFAULT 0 NOT NULL, expiry_date date NULL, purchase_id int8 NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT product_lots_pkey PRIMARY KEY (id), CONSTRAINT product_lots_qty_on_hand_check CHECK ((qty_on_hand >= (0)::numeric)), CONSTRAINT product_lots_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE CASCADE, CONSTRAINT product_lots_purchase_id_fkey FOREIGN KEY (purchase_id) REFERENCES pharmacy.purchases(id) ON DELETE SET NULL);
CREATE INDEX idx_product_lots_product_id ON pharmacy.product_lots USING btree (product_id);

-- Permissions

ALTER TABLE pharmacy.product_lots OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.product_lots TO postgres;


-- pharmacy.purchase_items definition

-- Drop table

-- DROP TABLE pharmacy.purchase_items;

CREATE TABLE pharmacy.purchase_items ( id int8 DEFAULT nextval('pharmacy.seq_purchase_items'::regclass) NOT NULL, purchase_id int8 NOT NULL, product_id int8 NOT NULL, lot_id int8 NULL, qty numeric(14, 4) NOT NULL, unit_cost numeric(14, 4) NOT NULL, discount numeric(14, 2) DEFAULT 0.0 NULL, tax_amount numeric(14, 2) DEFAULT 0.0 NULL, line_total numeric(14, 2) DEFAULT 0.0 NOT NULL, CONSTRAINT purchase_items_pkey PRIMARY KEY (id), CONSTRAINT purchase_items_qty_check CHECK ((qty > (0)::numeric)), CONSTRAINT purchase_items_unit_cost_check CHECK ((unit_cost >= (0)::numeric)), CONSTRAINT purchase_items_lot_id_fkey FOREIGN KEY (lot_id) REFERENCES pharmacy.product_lots(id) ON DELETE SET NULL, CONSTRAINT purchase_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE RESTRICT, CONSTRAINT purchase_items_purchase_id_fkey FOREIGN KEY (purchase_id) REFERENCES pharmacy.purchases(id) ON DELETE CASCADE);
CREATE INDEX idx_purchase_items_purchase_id ON pharmacy.purchase_items USING btree (purchase_id);

-- Permissions

ALTER TABLE pharmacy.purchase_items OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.purchase_items TO postgres;


-- pharmacy.purchase_payments definition

-- Drop table

-- DROP TABLE pharmacy.purchase_payments;

CREATE TABLE pharmacy.purchase_payments ( id int8 DEFAULT nextval('pharmacy.seq_purchase_payments'::regclass) NOT NULL, purchase_id int8 NOT NULL, amount numeric(14, 2) NOT NULL, method_id int8 NULL, paid_at timestamptz DEFAULT now() NOT NULL, reference text NULL, CONSTRAINT purchase_payments_amount_check CHECK ((amount >= (0)::numeric)), CONSTRAINT purchase_payments_pkey PRIMARY KEY (id), CONSTRAINT purchase_payments_method_id_fkey FOREIGN KEY (method_id) REFERENCES pharmacy.payment_methods(id), CONSTRAINT purchase_payments_purchase_id_fkey FOREIGN KEY (purchase_id) REFERENCES pharmacy.purchases(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.purchase_payments OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.purchase_payments TO postgres;


-- pharmacy.sale_items definition

-- Drop table

-- DROP TABLE pharmacy.sale_items;

CREATE TABLE pharmacy.sale_items ( id int8 DEFAULT nextval('pharmacy.seq_sale_items'::regclass) NOT NULL, sale_id int8 NOT NULL, product_id int8 NOT NULL, lot_id int8 NULL, qty numeric(14, 4) NOT NULL, unit_price numeric(14, 4) NOT NULL, discount numeric(14, 2) DEFAULT 0.0 NULL, tax_amount numeric(14, 2) DEFAULT 0.0 NULL, line_total numeric(14, 2) DEFAULT 0.0 NOT NULL, CONSTRAINT sale_items_pkey PRIMARY KEY (id), CONSTRAINT sale_items_qty_check CHECK ((qty > (0)::numeric)), CONSTRAINT sale_items_unit_price_check CHECK ((unit_price >= (0)::numeric)), CONSTRAINT sale_items_lot_id_fkey FOREIGN KEY (lot_id) REFERENCES pharmacy.product_lots(id) ON DELETE SET NULL, CONSTRAINT sale_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE RESTRICT, CONSTRAINT sale_items_sale_id_fkey FOREIGN KEY (sale_id) REFERENCES pharmacy.sales(id) ON DELETE CASCADE);
CREATE INDEX idx_sale_items_sale_id ON pharmacy.sale_items USING btree (sale_id);

-- Table Triggers

create trigger trg_sale_item_after_insert after
insert
    on
    pharmacy.sale_items for each row execute function pharmacy.fn_sale_item_after_insert();

-- Permissions

ALTER TABLE pharmacy.sale_items OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.sale_items TO postgres;


-- pharmacy.sale_payments definition

-- Drop table

-- DROP TABLE pharmacy.sale_payments;

CREATE TABLE pharmacy.sale_payments ( id int8 DEFAULT nextval('pharmacy.seq_sale_payments'::regclass) NOT NULL, sale_id int8 NULL, amount numeric(14, 2) NOT NULL, method_id int8 NULL, paid_at timestamptz DEFAULT now() NOT NULL, reference text NULL, CONSTRAINT sale_payments_amount_check CHECK ((amount >= (0)::numeric)), CONSTRAINT sale_payments_pkey PRIMARY KEY (id), CONSTRAINT sale_payments_method_id_fkey FOREIGN KEY (method_id) REFERENCES pharmacy.payment_methods(id), CONSTRAINT sale_payments_sale_id_fkey FOREIGN KEY (sale_id) REFERENCES pharmacy.sales(id) ON DELETE CASCADE);
CREATE INDEX idx_sale_payments_sale_id ON pharmacy.sale_payments USING btree (sale_id) INCLUDE (amount, paid_at);

-- Permissions

ALTER TABLE pharmacy.sale_payments OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.sale_payments TO postgres;


-- pharmacy.inventory_movements definition

-- Drop table

-- DROP TABLE pharmacy.inventory_movements;

CREATE TABLE pharmacy.inventory_movements ( id int8 DEFAULT nextval('pharmacy.seq_inventory_movements'::regclass) NOT NULL, product_id int8 NOT NULL, lot_id int8 NULL, location_id int8 NULL, change_qty numeric(14, 4) NOT NULL, reason text NOT NULL, reference_type text NULL, reference_id int8 NULL, "cost" numeric(14, 2) NULL, created_at timestamptz DEFAULT now() NOT NULL, created_by int8 NULL, CONSTRAINT inventory_movements_pkey PRIMARY KEY (id), CONSTRAINT inventory_movements_created_by_fkey FOREIGN KEY (created_by) REFERENCES pharmacy.users(id) ON DELETE SET NULL, CONSTRAINT inventory_movements_location_id_fkey FOREIGN KEY (location_id) REFERENCES pharmacy.inventory_locations(id) ON DELETE SET NULL, CONSTRAINT inventory_movements_lot_id_fkey FOREIGN KEY (lot_id) REFERENCES pharmacy.product_lots(id) ON DELETE SET NULL, CONSTRAINT inventory_movements_product_id_fkey FOREIGN KEY (product_id) REFERENCES pharmacy.products(id) ON DELETE RESTRICT);
CREATE INDEX idx_inventory_lot ON pharmacy.inventory_movements USING btree (lot_id);
CREATE INDEX idx_inventory_movements_created_at ON pharmacy.inventory_movements USING btree (created_at);
CREATE INDEX idx_inventory_movements_product_id ON pharmacy.inventory_movements USING btree (product_id);
CREATE INDEX idx_inventory_product ON pharmacy.inventory_movements USING btree (product_id);

-- Table Triggers

create trigger trg_check_lot_stock_before_insert before
insert
    on
    pharmacy.inventory_movements for each row execute function pharmacy.fn_check_lot_stock_before_insert();
create trigger trg_update_lot_after_insert after
insert
    on
    pharmacy.inventory_movements for each row execute function pharmacy.fn_update_lot_on_movement();

-- Permissions

ALTER TABLE pharmacy.inventory_movements OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.inventory_movements TO postgres;


-- pharmacy.sale_payment_allocations definition

-- Drop table

-- DROP TABLE pharmacy.sale_payment_allocations;

CREATE TABLE pharmacy.sale_payment_allocations ( id int8 DEFAULT nextval('pharmacy.seq_sale_payment_allocations'::regclass) NOT NULL, payment_id int8 NOT NULL, credit_invoice_id int8 NULL, amount numeric(14, 2) NOT NULL, CONSTRAINT sale_payment_allocations_amount_check CHECK ((amount >= (0)::numeric)), CONSTRAINT sale_payment_allocations_pkey PRIMARY KEY (id), CONSTRAINT sale_payment_allocations_credit_invoice_id_fkey FOREIGN KEY (credit_invoice_id) REFERENCES pharmacy.sales(id) ON DELETE SET NULL, CONSTRAINT sale_payment_allocations_payment_id_fkey FOREIGN KEY (payment_id) REFERENCES pharmacy.sale_payments(id) ON DELETE CASCADE);

-- Permissions

ALTER TABLE pharmacy.sale_payment_allocations OWNER TO postgres;
GRANT ALL ON TABLE pharmacy.sale_payment_allocations TO postgres;



-- DROP FUNCTION pharmacy."fn_best-selling_products_current_month"(timestamptz, timestamptz);

CREATE OR REPLACE FUNCTION pharmacy."fn_best-selling_products_current_month"(p_start timestamp with time zone, p_end timestamp with time zone)
 RETURNS TABLE(ranking bigint, product_id bigint, sku text, producto text, marca text, categoria text, qty_vendida numeric, num_ventas bigint, num_clientes bigint, precio_promedio numeric, costo_unitario numeric, ingresos_totales numeric, descuentos_aplicados numeric, impuestos_generados numeric, margen_bruto_pct numeric, ultima_venta timestamp with time zone)
 LANGUAGE plpgsql
AS $function$
BEGIN
  RETURN QUERY
  SELECT
    RANK() OVER (ORDER BY SUM(si.qty) DESC)::bigint           AS ranking,
    p.id                                                       AS product_id,
    p.sku,
    p.name                                                     AS producto,
    p.brand                                                    AS marca,
    cat.name                                                   AS categoria,
    SUM(si.qty)                                                AS qty_vendida,
    COUNT(DISTINCT si.sale_id)                                 AS num_ventas,
    COUNT(DISTINCT s.customer_id)                              AS num_clientes,
    ROUND(AVG(si.unit_price), 4)                               AS precio_promedio,
    p.default_cost                                             AS costo_unitario,
    SUM(si.line_total)                                         AS ingresos_totales,
    SUM(si.discount)                                           AS descuentos_aplicados,
    SUM(si.tax_amount)                                         AS impuestos_generados,
    ROUND(
      (SUM(si.line_total) - (p.default_cost * SUM(si.qty)))
      / NULLIF(SUM(si.line_total), 0) * 100, 2)               AS margen_bruto_pct,
    MAX(s.date)                                                AS ultima_venta
  FROM sale_items si
  JOIN sales     s   ON s.id  = si.sale_id
                     AND s.status = 'completed'
                     AND s.date  BETWEEN p_start AND p_end
  JOIN products  p   ON p.id  = si.product_id
                     AND p.deleted_at IS NULL
  LEFT JOIN categories cat ON cat.id = p.category_id
  GROUP BY p.id, p.sku, p.name, p.brand, p.default_cost, cat.name
  ORDER BY qty_vendida DESC;
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy."fn_best-selling_products_current_month"(timestamptz, timestamptz) OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy."fn_best-selling_products_current_month"(timestamptz, timestamptz) TO postgres;

-- DROP FUNCTION pharmacy.fn_check_lot_stock_before_insert();

CREATE OR REPLACE FUNCTION pharmacy.fn_check_lot_stock_before_insert()
 RETURNS trigger
 LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.lot_id IS NOT NULL AND NEW.change_qty < 0 THEN
    PERFORM 1 FROM product_lots WHERE id = NEW.lot_id FOR UPDATE;
    IF (SELECT qty_on_hand FROM product_lots WHERE id = NEW.lot_id) + NEW.change_qty < 0 THEN
      RAISE EXCEPTION 'Insufficient stock in lot %', NEW.lot_id;
    END IF;
  END IF;
  RETURN NEW;
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy.fn_check_lot_stock_before_insert() OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy.fn_check_lot_stock_before_insert() TO postgres;

-- DROP FUNCTION pharmacy.fn_revert_stock_on_sale_cancel();

CREATE OR REPLACE FUNCTION pharmacy.fn_revert_stock_on_sale_cancel()
 RETURNS trigger
 LANGUAGE plpgsql
AS $function$
DECLARE
  rec RECORD;
  already boolean;
BEGIN
  -- Solo actúa cuando el status cambia a 'CANCEL'
  IF TG_OP = 'UPDATE' THEN
    IF OLD.status IS DISTINCT FROM NEW.status AND NEW.status = 'CANCEL' THEN
      -- evitar reversión doble
      SELECT EXISTS (SELECT 1 FROM inventory_movements WHERE reason = 'sale_cancel' AND reference_type = 'sale' AND reference_id = NEW.id) INTO already;
      IF already THEN
        RETURN NEW;
      END IF;
      FOR rec IN SELECT product_id, lot_id, qty, unit_price FROM sale_items WHERE sale_id = NEW.id LOOP
        INSERT INTO inventory_movements(product_id, lot_id, change_qty, reason, reference_type, reference_id, cost, created_by)
        VALUES (rec.product_id, rec.lot_id, rec.qty, 'sale_cancel', 'sale', NEW.id, rec.unit_price, NULL);
      END LOOP;
    END IF;
  END IF;
  RETURN NEW;
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy.fn_revert_stock_on_sale_cancel() OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy.fn_revert_stock_on_sale_cancel() TO postgres;

-- DROP FUNCTION pharmacy.fn_sale_item_after_insert();

CREATE OR REPLACE FUNCTION pharmacy.fn_sale_item_after_insert()
 RETURNS trigger
 LANGUAGE plpgsql
AS $function$
BEGIN
  INSERT INTO inventory_movements(product_id, lot_id, change_qty, reason, reference_type, reference_id, cost, created_by)
  VALUES (NEW.product_id, NEW.lot_id, -NEW.qty, 'sale', 'sale', NEW.sale_id, NEW.unit_price, NULL);
  RETURN NEW;
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy.fn_sale_item_after_insert() OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy.fn_sale_item_after_insert() TO postgres;

-- DROP FUNCTION pharmacy.fn_update_lot_on_movement();

CREATE OR REPLACE FUNCTION pharmacy.fn_update_lot_on_movement()
 RETURNS trigger
 LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.lot_id IS NOT NULL THEN
    UPDATE product_lots SET qty_on_hand = qty_on_hand + NEW.change_qty WHERE id = NEW.lot_id;
  END IF;
  RETURN NEW;
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy.fn_update_lot_on_movement() OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy.fn_update_lot_on_movement() TO postgres;

-- DROP FUNCTION pharmacy.fn_write_audit_log(text, text, int8, text, int8, jsonb);

CREATE OR REPLACE FUNCTION pharmacy.fn_write_audit_log(p_entity_type text, p_table_name text, p_entity_id bigint, p_action text, p_changed_by bigint, p_change_data jsonb)
 RETURNS void
 LANGUAGE plpgsql
AS $function$
BEGIN
  INSERT INTO audit_log(entity_type, table_name, entity_id, action, changed_by, changed_at, change_data)
  VALUES (p_entity_type, p_table_name, p_entity_id, p_action, p_changed_by, now(), p_change_data);
END;
$function$
;

-- Permissions

ALTER FUNCTION pharmacy.fn_write_audit_log(text, text, int8, text, int8, jsonb) OWNER TO postgres;
GRANT ALL ON FUNCTION pharmacy.fn_write_audit_log(text, text, int8, text, int8, jsonb) TO postgres;


-- Permissions

GRANT ALL ON SCHEMA pharmacy TO postgres;
GRANT ALL ON SCHEMA pharmacy TO deupback;

