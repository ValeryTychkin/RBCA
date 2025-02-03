use crate::{
    query::organization as org_query,
    schema::{base::Pagination, organization as org_schema},
};
use orm_addons_lib::prelude::EntityFilterableTrait;
use repository_db_lib::organization::{org_entity, Organization as OrgRep, Repository};
use sea_orm::ActiveValue;
use uuid::Uuid;

pub async fn get_by_id(id: Uuid) -> Option<org_schema::Organization> {
    // Get model
    let rep = OrgRep::new().await;
    let model = rep.get_by_id(id).await.unwrap();
    // Convert model to schema
    match model {
        Some(v) => Some(org_schema::Organization {
            id: v.id,
            display_name: v.display_name,
            is_deleted: v.is_deleted,
            created_at: v.created_at,
        }),
        None => None,
    }
}

pub async fn get_all(query_filter: org_query::Organization) -> org_schema::OrganizationList {
    //Get filter
    let filter = query_filter.to_condition::<org_entity::Entity>();
    // Get models by filter
    let rep = OrgRep::new().await;
    let (models, offset, limit, total_count) = rep
        .get_multiple(
            Some(filter.to_owned()),
            query_filter.offset,
            query_filter.limit,
            true,
        )
        .await
        .unwrap();
    // Convert Model to Schema
    let mut orgs: Vec<org_schema::Organization> = vec![];
    for model in models {
        orgs.push(org_schema::Organization {
            id: model.id,
            display_name: model.display_name,
            is_deleted: model.is_deleted,
            created_at: model.created_at,
        });
    }
    org_schema::OrganizationList {
        organizations: orgs,
        pagination: Pagination {
            limit,
            offset,
            total: total_count,
        },
    }
}

pub async fn create_new(org: org_schema::OrganizationCreate) -> org_schema::Organization {
    // Get Model
    let rep = OrgRep::new().await;
    let active_model = org_entity::ActiveModel {
        display_name: ActiveValue::Set(org.display_name),
        ..Default::default()
    };
    let model = rep.create(active_model).await.unwrap();
    // Convert Model to Schema
    org_schema::Organization {
        id: model.id,
        display_name: model.display_name,
        is_deleted: model.is_deleted,
        created_at: model.created_at,
    }
}
