table! {
    api_keys (id) {
        id -> Integer,
        api_key -> Text,
        user_id -> Integer,
        expires -> Timestamp,
        last_used -> Nullable<Timestamp>,
        row_created_timestamp -> Timestamp,
        key_type -> Text,
    }
}

table! {
    batteries (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        used_in -> Nullable<Text>,
        charge_interval_days -> Integer,
        row_created_timestamp -> Timestamp,
        active -> Bool,
    }
}

table! {
    battery_charge_cycles (id) {
        id -> Integer,
        battery_id -> Integer,
        tracked_time -> Timestamp,
        row_created_timestamp -> Timestamp,
        undone_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    chores (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        period_type -> Text,
        period_days -> Nullable<Integer>,
        row_created_timestamp -> Timestamp,
        period_config -> Nullable<Text>,
        rollover -> Nullable<Bool>,
        assignment_type -> Nullable<Text>,
        assignment_config -> Nullable<Text>,
        next_execution_assigned_to_user_id -> Nullable<Integer>,
        consume_product_on_execution -> Bool,
        product_id -> Nullable<Integer>,
        product_amount -> Nullable<Double>,
        period_interval -> Integer,
        active -> Bool,
    }
}

table! {
    chores_log (id) {
        id -> Integer,
        chore_id -> Integer,
        tracked_time -> Timestamp,
        done_by_user_id -> Integer,
        row_created_timestamp -> Timestamp,
        undone_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    equipment (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        instruction_manual_file_name -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    locations (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
        is_freezer -> Bool,
    }
}

table! {
    meal_plan (id) {
        id -> Integer,
        day -> Date,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        recipe_id -> Nullable<Integer>,
        recipe_servings -> Nullable<Integer>,
        note -> Nullable<Text>,
        product_id -> Nullable<Integer>,
        product_amount -> Nullable<Double>,
        product_qu_id -> Nullable<Integer>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    permission_hierarchy (id) {
        id -> Integer,
        name -> Text,
        parent -> Nullable<Integer>,
    }
}

table! {
    product_barcodes (id) {
        id -> Integer,
        product_id -> Integer,
        barcode -> Text,
        qu_id -> Nullable<Integer>,
        amount -> Nullable<Double>,
        shopping_location_id -> Nullable<Integer>,
        last_price -> Nullable<Double>,
        row_created_timestamp -> Timestamp,
        note -> Nullable<Text>,
    }
}

table! {
    product_groups (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        product_group_id -> Nullable<Integer>,
        active -> Bool,
        location_id -> Integer,
        shopping_location_id -> Nullable<Integer>,
        qu_id_purchase -> Integer,
        qu_id_stock -> Integer,
        qu_factor_purchase_to_stock -> Double,
        min_stock_amount -> Integer,
        default_best_before_days -> Integer,
        default_best_before_days_after_open -> Integer,
        default_best_before_days_after_freezing -> Integer,
        default_best_before_days_after_thawing -> Integer,
        picture_file_name -> Nullable<Text>,
        tare_weight -> Double,
        not_check_stock_fulfillment_for_recipes -> Nullable<Bool>,
        parent_product_id -> Nullable<Integer>,
        calories -> Nullable<Integer>,
        cumulate_min_stock_amount_of_sub_products -> Bool,
        due_type -> Integer,
        quick_consume_amount -> Double,
        hide_on_stock_overview -> Bool,
        row_created_timestamp -> Timestamp,
        default_print_stock_label -> Integer,
        allow_label_per_unit -> Integer,
    }
}

table! {
    quantity_unit_conversions (id) {
        id -> Integer,
        from_qu_id -> Integer,
        to_qu_id -> Integer,
        factor -> Double,
        product_id -> Nullable<Integer>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    quantity_units (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
        name_plural -> Nullable<Text>,
        plural_forms -> Nullable<Text>,
    }
}

table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
        picture_file_name -> Nullable<Text>,
        base_servings -> Nullable<Integer>,
        desired_servings -> Nullable<Integer>,
        not_check_shoppinglist -> Bool,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        product_id -> Nullable<Integer>,
    }
}

table! {
    recipes_nestings (id) {
        id -> Integer,
        recipe_id -> Integer,
        includes_recipe_id -> Integer,
        row_created_timestamp -> Timestamp,
        servings -> Nullable<Integer>,
    }
}

table! {
    recipes_pos (id) {
        id -> Integer,
        recipe_id -> Integer,
        product_id -> Integer,
        amount -> Double,
        note -> Nullable<Text>,
        qu_id -> Nullable<Integer>,
        only_check_single_unit_in_stock -> Bool,
        ingredient_group -> Nullable<Text>,
        not_check_stock_fulfillment -> Bool,
        row_created_timestamp -> Timestamp,
        variable_amount -> Nullable<Text>,
        price_factor -> Double,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        session_key -> Text,
        user_id -> Integer,
        expires -> Timestamp,
        last_used -> Timestamp,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    shopping_list (id) {
        id -> Integer,
        product_id -> Nullable<Integer>,
        note -> Nullable<Text>,
        amount -> Double,
        row_created_timestamp -> Timestamp,
        shopping_list_id -> Nullable<Integer>,
        done -> Bool,
        qu_id -> Nullable<Integer>,
    }
}

table! {
    shopping_lists (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    shopping_locations (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    stock (id) {
        id -> Integer,
        product_id -> Integer,
        amount -> Double,
        best_before_date -> Nullable<Date>,
        purchased_date -> Nullable<Date>,
        stock_id -> Text,
        price -> Nullable<Double>,
        opened_date -> Nullable<Date>,
        row_created_timestamp -> Timestamp,
        location_id -> Nullable<Integer>,
        shopping_location_id -> Nullable<Integer>,
    }
}

table! {
    stock_log (id) {
        id -> Integer,
        product_id -> Integer,
        amount -> Double,
        best_before_date -> Nullable<Date>,
        purchased_date -> Nullable<Date>,
        used_date -> Nullable<Date>,
        spoiled -> Bool,
        stock_id -> Text,
        transaction_type -> Text,
        price -> Nullable<Double>,
        undone_timestamp -> Nullable<Timestamp>,
        opened_date -> Nullable<Timestamp>,
        row_created_timestamp -> Timestamp,
        location_id -> Nullable<Integer>,
        recipe_id -> Nullable<Integer>,
        correlation_id -> Nullable<Text>,
        transaction_id -> Nullable<Text>,
        stock_row_id -> Nullable<Integer>,
        shopping_location_id -> Nullable<Integer>,
        user_id -> Integer,
    }
}

table! {
    task_categories (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        done_timestamp -> Nullable<Timestamp>,
        category_id -> Nullable<Integer>,
        assigned_to_user_id -> Nullable<Integer>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    user_permissions (permission_id, user_id) {
        permission_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    user_settings (id) {
        id -> Integer,
        user_id -> Integer,
        key -> Text,
        value -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
        row_updated_timestamp -> Timestamp,
    }
}

table! {
    userentities (id) {
        id -> Integer,
        name -> Text,
        caption -> Text,
        description -> Nullable<Text>,
        show_in_sidebar_menu -> Bool,
        icon_css_class -> Nullable<Text>,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    userfield_values (field_id, object_id) {
        field_id -> Integer,
        object_id -> Integer,
        value -> Text,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    userfields (id) {
        id -> Integer,
        entity -> Text,
        name -> Text,
        caption -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        show_as_column_in_tables -> Bool,
        row_created_timestamp -> Timestamp,
        config -> Nullable<Text>,
        sort_number -> Nullable<Integer>,
    }
}

table! {
    userobjects (id) {
        id -> Integer,
        userentity_id -> Integer,
        row_created_timestamp -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        password -> Text,
        row_created_timestamp -> Timestamp,
        picture_file_name -> Nullable<Text>,
    }
}

joinable!(api_keys -> users (user_id));
joinable!(battery_charge_cycles -> batteries (battery_id));
joinable!(chores -> products (product_id));
joinable!(chores -> users (next_execution_assigned_to_user_id));
joinable!(chores_log -> chores (chore_id));
joinable!(chores_log -> users (done_by_user_id));
joinable!(meal_plan -> products (product_id));
joinable!(meal_plan -> quantity_units (product_qu_id));
joinable!(meal_plan -> recipes (recipe_id));
joinable!(product_barcodes -> products (product_id));
joinable!(product_barcodes -> quantity_units (qu_id));
joinable!(product_barcodes -> shopping_locations (shopping_location_id));
joinable!(products -> locations (location_id));
joinable!(products -> product_groups (product_group_id));
joinable!(products -> shopping_locations (shopping_location_id));
joinable!(quantity_unit_conversions -> products (product_id));
joinable!(recipes -> products (product_id));
joinable!(recipes_pos -> products (product_id));
joinable!(recipes_pos -> quantity_units (qu_id));
joinable!(recipes_pos -> recipes (recipe_id));
joinable!(sessions -> users (user_id));
joinable!(shopping_list -> products (product_id));
joinable!(shopping_list -> quantity_units (qu_id));
joinable!(shopping_list -> shopping_lists (shopping_list_id));
joinable!(stock -> locations (location_id));
joinable!(stock -> products (product_id));
joinable!(stock -> shopping_locations (shopping_location_id));
joinable!(stock_log -> locations (location_id));
joinable!(stock_log -> products (product_id));
joinable!(stock_log -> recipes (recipe_id));
joinable!(stock_log -> shopping_locations (shopping_location_id));
joinable!(stock_log -> users (user_id));
joinable!(tasks -> task_categories (category_id));
joinable!(tasks -> users (assigned_to_user_id));
joinable!(user_permissions -> permission_hierarchy (permission_id));
joinable!(user_permissions -> users (user_id));
joinable!(user_settings -> users (user_id));
joinable!(userfield_values -> userfields (field_id));

allow_tables_to_appear_in_same_query!(
    api_keys,
    batteries,
    battery_charge_cycles,
    chores,
    chores_log,
    equipment,
    locations,
    meal_plan,
    permission_hierarchy,
    product_barcodes,
    product_groups,
    products,
    quantity_unit_conversions,
    quantity_units,
    recipes,
    recipes_nestings,
    recipes_pos,
    sessions,
    shopping_list,
    shopping_lists,
    shopping_locations,
    stock,
    stock_log,
    task_categories,
    tasks,
    user_permissions,
    user_settings,
    userentities,
    userfield_values,
    userfields,
    userobjects,
    users,
);
