use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // sales
        manager
            .create_index(
                Index::create()
                    .name("idx_sales_customer_id")
                    .table(Sales::Table)
                    .col(Sales::CustomerId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_sales_user_id")
                    .table(Sales::Table)
                    .col(Sales::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_sales_status")
                    .table(Sales::Table)
                    .col(Sales::Status)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_sales_date")
                    .table(Sales::Table)
                    .col(Sales::Date)
                    .to_owned(),
            )
            .await?;

        // sale_items
        manager
            .create_index(
                Index::create()
                    .name("idx_sale_items_sale_id")
                    .table(SaleItems::Table)
                    .col(SaleItems::SaleId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_sale_items_product_id")
                    .table(SaleItems::Table)
                    .col(SaleItems::ProductId)
                    .to_owned(),
            )
            .await?;

        // sale_payments
        manager
            .create_index(
                Index::create()
                    .name("idx_sale_payments_sale_id")
                    .table(SalePayments::Table)
                    .col(SalePayments::SaleId)
                    .to_owned(),
            )
            .await?;

        // inventory_movements
        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_movements_product_id")
                    .table(InventoryMovements::Table)
                    .col(InventoryMovements::ProductId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_movements_lot_id")
                    .table(InventoryMovements::Table)
                    .col(InventoryMovements::LotId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_movements_created_at")
                    .table(InventoryMovements::Table)
                    .col(InventoryMovements::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // product_lots
        manager
            .create_index(
                Index::create()
                    .name("idx_product_lots_product_id")
                    .table(ProductLots::Table)
                    .col(ProductLots::ProductId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_product_lots_expiry_date")
                    .table(ProductLots::Table)
                    .col(ProductLots::ExpiryDate)
                    .to_owned(),
            )
            .await?;

        // purchase_items
        manager
            .create_index(
                Index::create()
                    .name("idx_purchase_items_purchase_id")
                    .table(PurchaseItems::Table)
                    .col(PurchaseItems::PurchaseId)
                    .to_owned(),
            )
            .await?;

        // categories
        manager
            .create_index(
                Index::create()
                    .name("idx_categories_name")
                    .table(Categories::Table)
                    .col(Categories::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_categories_parent_id")
                    .table(Categories::Table)
                    .col(Categories::ParentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_categories_parent_id")
                    .table(Categories::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_categories_name")
                    .table(Categories::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_purchase_items_purchase_id")
                    .table(PurchaseItems::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_lots_expiry_date")
                    .table(ProductLots::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_lots_product_id")
                    .table(ProductLots::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_inventory_movements_created_at")
                    .table(InventoryMovements::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_inventory_movements_lot_id")
                    .table(InventoryMovements::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_inventory_movements_product_id")
                    .table(InventoryMovements::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sale_payments_sale_id")
                    .table(SalePayments::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sale_items_product_id")
                    .table(SaleItems::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sale_items_sale_id")
                    .table(SaleItems::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sales_date")
                    .table(Sales::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sales_status")
                    .table(Sales::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sales_user_id")
                    .table(Sales::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sales_customer_id")
                    .table(Sales::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Sales {
    Table,
    CustomerId,
    UserId,
    Status,
    Date,
}

#[derive(Iden)]
enum SaleItems {
    Table,
    SaleId,
    ProductId,
}

#[derive(Iden)]
enum SalePayments {
    Table,
    SaleId,
}

#[derive(Iden)]
enum InventoryMovements {
    Table,
    ProductId,
    LotId,
    CreatedAt,
}

#[derive(Iden)]
enum ProductLots {
    Table,
    ProductId,
    ExpiryDate,
}

#[derive(Iden)]
enum PurchaseItems {
    Table,
    PurchaseId,
}

#[derive(Iden)]
enum Categories {
    Table,
    Name,
    ParentId,
}
