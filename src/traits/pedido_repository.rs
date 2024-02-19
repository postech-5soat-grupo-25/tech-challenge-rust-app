use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::{
    cliente::Cliente,
    pedido::{Pedido, Status},
    produto::Produto,
};

#[automock]
#[async_trait]
pub trait PedidoRepository {
    async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError>;

    async fn lista_pedidos(&mut self) -> Result<Vec<Pedido>, DomainError>;

    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError>;

    async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError>;

    async fn cadastrar_cliente(
        &mut self,
        pedido_id: usize,
        cliente: Cliente,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_lanche(
        &mut self,
        pedido_id: usize,
        lanche: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_acompanhamento(
        &mut self,
        pedido_id: usize,
        acompanhamento: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_bebida(
        &mut self,
        pedido_id: usize,
        bebida: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_pagamento(
        &mut self,
        pedido_id: usize,
        pagamento_id: String,
    ) -> Result<Pedido, DomainError>;

    async fn atualiza_status(
        &mut self,
        pedido_id: usize,
        status: Status,
    ) -> Result<Pedido, DomainError>;
}