use chrono::{DateTime, Utc};
use futures::future::try_join_all;
use std::collections::HashMap;

use super::{
    auth_service::AuthRepository, dto::order::OrderDto, map_service::MapRepository,
    tow_truck_service::TowTruckRepository,
};
use crate::{errors::AppError, models::order::Order};

pub trait OrderRepository {
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError>;
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError>;
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError>;
    async fn create_order(
        &self,
        customer_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError>;
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError>;
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError>;
}

#[derive(Debug)]
pub struct OrderService<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> {
    order_repository: T,
    tow_truck_repository: U,
    auth_repository: V,
    map_repository: W,
}

impl<
        T: OrderRepository + std::fmt::Debug,
        U: TowTruckRepository + std::fmt::Debug,
        V: AuthRepository + std::fmt::Debug,
        W: MapRepository + std::fmt::Debug,
    > OrderService<T, U, V, W>
{
    pub fn new(
        order_repository: T,
        tow_truck_repository: U,
        auth_repository: V,
        map_repository: W,
    ) -> Self {
        OrderService {
            order_repository,
            tow_truck_repository,
            auth_repository,
            map_repository,
        }
    }

    pub async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        self.order_repository
            .update_order_status(order_id, status)
            .await
    }

    pub async fn get_order_by_id(&self, id: i32) -> Result<OrderDto, AppError> {
        let order = self.order_repository.find_order_by_id(id).await?;

        let client_username = self
            .auth_repository
            .find_user_by_id(order.client_id)
            .await
            .unwrap()
            .unwrap()
            .username;

        let dispatcher = match order.dispatcher_id {
            Some(dispatcher_id) => self
                .auth_repository
                .find_dispatcher_by_id(dispatcher_id)
                .await
                .unwrap(),
            None => None,
        };

        let (dispatcher_user_id, dispatcher_username) = match dispatcher {
            Some(dispatcher) => (
                Some(dispatcher.user_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(dispatcher.user_id)
                        .await
                        .unwrap()
                        .unwrap()
                        .username,
                ),
            ),
            None => (None, None),
        };

        let tow_truck = match order.tow_truck_id {
            Some(tow_truck_id) => self
                .tow_truck_repository
                .find_tow_truck_by_id(tow_truck_id)
                .await
                .unwrap(),
            None => None,
        };

        let (driver_user_id, driver_username) = match tow_truck {
            Some(tow_truck) => (
                Some(tow_truck.driver_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(tow_truck.driver_id)
                        .await
                        .unwrap()
                        .unwrap()
                        .username,
                ),
            ),
            None => (None, None),
        };

        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await
            .unwrap();

        Ok(OrderDto {
            id: order.id,
            client_id: order.client_id,
            client_username: Some(client_username),
            dispatcher_user_id,
            dispatcher_username,
            driver_user_id,
            driver_username,
            area_id,
            dispatcher_id: order.dispatcher_id,
            tow_truck_id: order.tow_truck_id,
            status: order.status,
            node_id: order.node_id,
            car_value: order.car_value,
            order_time: order.order_time,
            completed_time: order.completed_time,
        })
    }

    pub async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderDto>, AppError> {
        let orders = self
            .order_repository
            .get_paginated_orders(page, page_size, sort_by, sort_order, status, area)
            .await?;
    
        // 必要なIDを一括取得
        let client_ids: Vec<i32> = orders.iter().map(|order| order.client_id).collect();
        let dispatcher_ids: Vec<i32> = orders
            .iter()
            .filter_map(|order| order.dispatcher_id)
            .collect();
        let tow_truck_ids: Vec<i32> = orders
            .iter()
            .filter_map(|order| order.tow_truck_id)
            .collect();
        let node_ids: Vec<i32> = orders.iter().map(|order| order.node_id).collect();
    
        // クライアントユーザー名を一括取得
        let clients = self
            .auth_repository
            .find_users_by_ids(&client_ids)
            .await?
            .into_iter()
            .map(|user| (user.id, user.username))
            .collect::<HashMap<i32, String>>();
    
        // ディスパッチャー情報を一括取得
        let dispatchers = self
            .auth_repository
            .find_dispatchers_by_ids(&dispatcher_ids)
            .await?
            .into_iter()
            .map(|dispatcher| (dispatcher.id, dispatcher))
            .collect::<HashMap<i32, _>>();
    
        let dispatcher_user_ids: Vec<i32> = dispatchers
            .values()
            .map(|dispatcher| dispatcher.user_id)
            .collect();
    
        let dispatcher_users = self
            .auth_repository
            .find_users_by_ids(&dispatcher_user_ids)
            .await?
            .into_iter()
            .map(|user| (user.id, user.username))
            .collect::<HashMap<i32, String>>();
    
        // トラック情報を一括取得
        let tow_trucks = self
            .tow_truck_repository
            .find_tow_trucks_by_ids(&tow_truck_ids)
            .await?
            .into_iter()
            .map(|truck| (truck.id, truck))
            .collect::<HashMap<i32, _>>();
    
        let driver_user_ids: Vec<i32> = tow_trucks
            .values()
            .map(|truck| truck.driver_id)
            .collect();
    
        let drivers = self
            .auth_repository
            .find_users_by_ids(&driver_user_ids)
            .await?
            .into_iter()
            .map(|user| (user.id, user.username))
            .collect::<HashMap<i32, String>>();
    
        // エリアIDを一括取得
        let area_ids = self
            .map_repository
            .get_area_ids_by_node_ids(&node_ids)
            .await?
            .into_iter()
            .collect::<HashMap<i32, i32>>();
    
        // OrderDtoを一括作成
        let results = orders
            .into_iter()
            .map(|order| {
                let client_username = clients.get(&order.client_id).cloned();
    
                let (dispatcher_user_id, dispatcher_username) = if let Some(dispatcher_id) = order.dispatcher_id {
                    if let Some(dispatcher) = dispatchers.get(&dispatcher_id) {
                        let username = dispatcher_users.get(&dispatcher.user_id).cloned();
                        (Some(dispatcher.user_id), username)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };
    
                let (driver_user_id, driver_username) = if let Some(tow_truck_id) = order.tow_truck_id {
                    if let Some(tow_truck) = tow_trucks.get(&tow_truck_id) {
                        let username = drivers.get(&tow_truck.driver_id).cloned();
                        (Some(tow_truck.driver_id), username)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };
    
                let area_id = area_ids.get(&order.node_id).cloned().unwrap_or(0);
    
                OrderDto {
                    id: order.id,
                    client_id: order.client_id,
                    client_username,
                    dispatcher_id: order.dispatcher_id,
                    dispatcher_user_id,
                    dispatcher_username,
                    tow_truck_id: order.tow_truck_id,
                    driver_user_id,
                    driver_username,
                    area_id,
                    status: order.status.clone(),
                    node_id: order.node_id,
                    car_value: order.car_value,
                    order_time: order.order_time,
                    completed_time: order.completed_time,
                }
            })
            .collect();
    
        Ok(results)
    }

    pub async fn create_client_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        match self
            .order_repository
            .create_order(client_id, node_id, car_value)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::BadRequest),
        }
    }

    pub async fn create_dispatcher_order(
        &self,
        order_id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
        order_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        if (self
            .order_repository
            .create_completed_order(order_id, tow_truck_id, order_time)
            .await)
            .is_err()
        {
            return Err(AppError::BadRequest);
        }

        self.order_repository
            .update_order_dispatched(order_id, dispatcher_id, tow_truck_id)
            .await?;

        self.tow_truck_repository
            .update_status(tow_truck_id, "busy")
            .await?;

        Ok(())
    }
}
