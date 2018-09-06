table! {
    categories (category_id) {
        category_id -> Int4,
        name -> Text,
    }
}

table! {
    containers (container_id) {
        container_id -> Int4,
        name -> Text,
    }
}

table! {
    ingredients (ingredient_id) {
        ingredient_id -> Int4,
        name -> Text,
    }
}

table! {
    instructions (instruction_id) {
        instruction_id -> Int4,
    }
}

table! {
    instruction_steps (instruction_id, step_id) {
        instruction_id -> Int4,
        step_id -> Int4,
    }
}

table! {
    recipe (recipe_id) {
        recipe_id -> Int4,
        name -> Text,
        category_id -> Nullable<Int4>,
    }
}

table! {
    steps (step_id) {
        step_id -> Int4,
        verb_id -> Nullable<Int4>,
        stop_when -> Nullable<Text>,
        stop_after -> Nullable<Float4>,
        container_id -> Nullable<Int4>,
        tool_id -> Nullable<Int4>,
    }
}

table! {
    tools (tool_id) {
        tool_id -> Int4,
        name -> Text,
    }
}

table! {
    units (unit_id) {
        unit_id -> Int4,
        name -> Text,
    }
}

table! {
    verb_objects (verb_object_id) {
        verb_object_id -> Int4,
        verb_id -> Nullable<Int4>,
        ingredient_id -> Nullable<Int4>,
        amount -> Float4,
        unit_id -> Nullable<Int4>,
    }
}

table! {
    verbs (verb_id) {
        verb_id -> Int4,
        verb -> Text,
    }
}

joinable!(instruction_steps -> instructions (instruction_id));
joinable!(instruction_steps -> steps (step_id));
joinable!(recipe -> categories (category_id));
joinable!(steps -> containers (container_id));
joinable!(steps -> tools (tool_id));
joinable!(steps -> verbs (verb_id));
joinable!(verb_objects -> ingredients (ingredient_id));
joinable!(verb_objects -> units (unit_id));
joinable!(verb_objects -> verbs (verb_id));

allow_tables_to_appear_in_same_query!(
    categories,
    containers,
    ingredients,
    instructions,
    instruction_steps,
    recipe,
    steps,
    tools,
    units,
    verb_objects,
    verbs,
);
