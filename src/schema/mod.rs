mod queries;
mod mutations;


use async_graphql::MergedObject;
pub use queries::*;
pub use mutations::*;



#[derive(MergedObject, Default)]
pub struct QueryRoot(
    UserQuery,
    CmsQuery
);


#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation,
    CMSMutation
);