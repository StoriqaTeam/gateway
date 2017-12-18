use juniper;

// Graphql context
pub struct Context;

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}
