use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_module::{
    sale_items::SaleItemDetailResponse,
    sale_payment_allocations::sale_payment_allocations_dto::sale_payment_allocations_dto::{
        SalePaymentAllocationDetailResponse, SalePaymentAllocationRequest,
    },
    sale_payments::{SalePaymentDetailResponse, SalePaymentRequest},
    sales::{SaleDetailResponse, SaleRequest},
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]

pub struct SaleAddRequest {
    pub customer_id: Option<i64>,
    pub user_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub is_credit: bool,
    pub created_at: DateTimeWithTimeZone,
    pub method_id: Option<i64>,
    pub reference: Option<String>,
    pub credit_invoice_id: Option<i64>,
    pub items: Vec<SaleAddItemRequest>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]

pub struct SaleAddItemRequest {
    pub id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub qty: Decimal,
    pub unit_price: Decimal,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleAddIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]

pub struct SaleAddDetailResponse {
    pub id: i64,
    pub customer_id: Option<i64>,
    pub user_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub is_credit: bool,
    pub created_at: DateTimeWithTimeZone,
    pub method_id: Option<i64>,
    pub reference: Option<String>,
    pub credit_invoice_id: Option<i64>,
    pub items: Vec<SaleItemDetailResponse>,
}

//Pasar de `&SaleAddRequest` a `SaleRequest` (usa referencia para no mover el payload)
impl From<&SaleAddRequest> for SaleRequest {
    fn from(request: &SaleAddRequest) -> Self {
        Self {
            customer_id: request.customer_id,
            user_id: request.user_id,
            invoice_no: request.invoice_no.clone(),
            date: request.date,
            subtotal: request.subtotal,
            tax_total: request.tax_total,
            discount_total: request.discount_total,
            total: request.total,
            status: request.status.clone(),
            is_credit: request.is_credit,
            created_at: request.created_at,
        }
    }
}

//Pasar de `( &SaleAddRequest, sale_id )` a `SalePaymentRequest` (usa referencia para no mover el payload)
impl From<(&SaleAddRequest, i64)> for SalePaymentRequest {
    fn from((request, sale_id): (&SaleAddRequest, i64)) -> Self {
        Self {
            sale_id: Some(sale_id),
            amount: request.total,
            method_id: request.method_id,
            paid_at: request.created_at,
            reference: request.reference.clone(),
        }
    }
}

//Pasar de `(&SaleAddRequest, sale_id)` a SalePaymentAllocationRequest
impl From<(&SaleAddRequest, i64, i64)> for SalePaymentAllocationRequest {
    fn from((request, payment_id, sale_id): (&SaleAddRequest, i64, i64)) -> Self {
        Self {
            payment_id: payment_id, // Usamos el mismo ID para el pago y la asignación
            credit_invoice_id: Some(sale_id), // Usamos el ID de la venta como referencia para la factura de crédito
            amount: request.total,
        }
    }
}

//Pasar cada SaleAddRequest Items a SaleItemRequest
impl From<(&SaleAddItemRequest, i64)>
    for crate::api_module::sale_items::sale_items_dto::sale_items_dto::SaleItemRequest
{
    fn from((request, sale_id): (&SaleAddItemRequest, i64)) -> Self {
        Self {
            id: request.id,
            sale_id: sale_id,
            product_id: request.product_id,
            lot_id: request.lot_id,
            qty: request.qty,
            unit_price: request.unit_price,
            discount: request.discount,
            tax_amount: request.tax_amount,
            line_total: request.line_total,
        }
    }
}

//Pasar de SaleDetailResponse, SaleItemDetailResponse,SalePaymentDetailResponse ,SalePaymentAllocationDetailResponse a SaleAddDetailResponse y items SaleItemDetailResponse
impl
    From<(
        SaleDetailResponse,
        Vec<SaleItemDetailResponse>,
        Option<SalePaymentDetailResponse>,
        Option<SalePaymentAllocationDetailResponse>,
    )> for SaleAddDetailResponse
{
    fn from(
        details: (
            SaleDetailResponse,
            Vec<SaleItemDetailResponse>,
            Option<SalePaymentDetailResponse>,
            Option<SalePaymentAllocationDetailResponse>,
        ),
    ) -> Self {
        let (sale_detail, items_detail, payment_detail, allocation_detail) = details;
        Self {
            id: sale_detail.id,
            customer_id: sale_detail.customer_id,
            user_id: sale_detail.user_id,
            invoice_no: sale_detail.invoice_no,
            date: sale_detail.date,
            subtotal: sale_detail.subtotal,
            tax_total: sale_detail.tax_total,
            discount_total: sale_detail.discount_total,
            total: sale_detail.total,
            status: sale_detail.status,
            is_credit: sale_detail.is_credit,
            created_at: sale_detail.created_at,
            method_id: payment_detail.as_ref().map(|p| p.method_id).flatten(),
            reference: payment_detail
                .as_ref()
                .map(|p| p.reference.clone())
                .flatten(),
            credit_invoice_id: allocation_detail
                .as_ref()
                .map(|a| a.credit_invoice_id)
                .flatten(),
            items: items_detail,
        }
    }
}
