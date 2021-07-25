table! {
    api_keys (id) {
        id -> Int4,
        api_key -> Text,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        last_used -> Nullable<Timestamp>,
        row_created_timestamp -> Nullable<Timestamp>,
        key_type -> Text,
    }
}

table! {
    batteries (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        used_in -> Nullable<Text>,
        charge_interval_days -> Int4,
        row_created_timestamp -> Nullable<Timestamp>,
        active -> Bool,
    }
}

table! {
    battery_charge_cycles (id) {
        id -> Int4,
        battery_id -> Text,
        tracked_time -> Nullable<Timestamp>,
        row_created_timestamp -> Nullable<Timestamp>,
        undone -> Bool,
        undone_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    chores (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        period_type -> Text,
        period_days -> Nullable<Int4>,
        row_created_timestamp -> Nullable<Timestamp>,
        period_config -> Nullable<Text>,
        track_date_only -> Nullable<Bool>,
        rollover -> Nullable<Bool>,
        assignment_type -> Nullable<Text>,
        assignment_config -> Nullable<Text>,
        next_execution_assigned_to_user_id -> Nullable<Int4>,
        consume_product_on_execution -> Bool,
        product_id -> Nullable<Bool>,
        product_amount -> Nullable<Float8>,
        period_interval -> Int4,
        active -> Bool,
    }
}

table! {
    chores_log (id) {
        id -> Int4,
        chore_id -> Int4,
        tracked_time -> Nullable<Timestamp>,
        done_by_user_id -> Nullable<Int4>,
        row_created_timestamp -> Nullable<Timestamp>,
        undone -> Bool,
        undone_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    equipment (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        instruction_manual_file_name -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    locations (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
        is_freezer -> Bool,
    }
}

table! {
    meal_plan (id) {
        id -> Int4,
        day -> Date,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        recipe_id -> Nullable<Int4>,
        recipe_servings -> Nullable<Int4>,
        note -> Nullable<Text>,
        product_id -> Nullable<Int4>,
        product_amount -> Nullable<Float8>,
        product_qu_id -> Nullable<Int4>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    permission_hierarchy (id) {
        id -> Int4,
        name -> Text,
        parent -> Nullable<Int4>,
    }
}

table! {
    product_barcodes (id) {
        id -> Int4,
        product_id -> Int4,
        barcode -> Text,
        qu_id -> Nullable<Int4>,
        amount -> Nullable<Float8>,
        shopping_location_id -> Nullable<Int4>,
        last_price -> Nullable<Float8>,
        row_created_timestamp -> Nullable<Timestamp>,
        note -> Nullable<Text>,
    }
}

table! {
    product_groups (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        product_group_id -> Nullable<Int4>,
        active -> Bool,
        location_id -> Int4,
        shopping_location_id -> Nullable<Int4>,
        qu_id_purchase -> Int4,
        qu_id_stock -> Int4,
        qu_factor_purchase_to_stock -> Float8,
        min_stock_amount -> Int4,
        default_best_before_days -> Int4,
        default_best_before_days_after_open -> Int4,
        default_best_before_days_after_freezing -> Int4,
        default_best_before_days_after_thawing -> Int4,
        picture_file_name -> Nullable<Text>,
        enable_tare_weight_handling -> Bool,
        tare_weight -> Float8,
        not_check_stock_fulfillment_for_recipes -> Nullable<Bool>,
        parent_product_id -> Nullable<Int4>,
        calories -> Nullable<Int4>,
        cumulate_min_stock_amount_of_sub_products -> Nullable<Bool>,
        due_type -> Bool,
        quick_consume_amount -> Float8,
        hide_on_stock_overview -> Bool,
        row_created_timestamp -> Nullable<Timestamp>,
        default_print_stock_label -> Int4,
        allow_label_per_unit -> Int4,
    }
}

table! {
    quantity_unit_conversions (id) {
        id -> Int4,
        from_qu_id -> Int4,
        to_qu_id -> Int4,
        factor -> Float8,
        product_id -> Nullable<Int4>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    quantity_units (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
        name_plural -> Nullable<Text>,
        plural_forms -> Nullable<Text>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
        picture_file_name -> Nullable<Text>,
        base_servings -> Nullable<Int4>,
        desired_servings -> Nullable<Int4>,
        not_check_shoppinglist -> Bool,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        product_id -> Nullable<Int4>,
    }
}

table! {
    recipes_nestings (id) {
        id -> Int4,
        recipe_id -> Int4,
        includes_recipe_id -> Int4,
        row_created_timestamp -> Nullable<Timestamp>,
        servings -> Nullable<Int4>,
    }
}

table! {
    recipes_pos (id) {
        id -> Int4,
        recipe_id -> Int4,
        product_id -> Int4,
        amount -> Float8,
        note -> Nullable<Text>,
        qu_id -> Nullable<Int4>,
        only_check_single_unit_in_stock -> Bool,
        ingredient_group -> Nullable<Text>,
        not_check_stock_fulfillment -> Bool,
        row_created_timestamp -> Nullable<Timestamp>,
        variable_amount -> Nullable<Text>,
        price_factor -> Float8,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        session_key -> Text,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        last_used -> Nullable<Timestamp>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    shopping_list (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        note -> Nullable<Text>,
        amount -> Float8,
        row_created_timestamp -> Nullable<Timestamp>,
        shopping_list_id -> Nullable<Int4>,
        done -> Nullable<Bool>,
        qu_id -> Nullable<Int4>,
    }
}

table! {
    shopping_lists (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    shopping_locations (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    stock (id) {
        id -> Int4,
        product_id -> Int4,
        amount -> Float8,
        best_before_date -> Nullable<Date>,
        purchased_date -> Nullable<Date>,
        stock_id -> Text,
        price -> Nullable<Float8>,
        open -> Bool,
        opened_date -> Nullable<Date>,
        row_created_timestamp -> Nullable<Timestamp>,
        location_id -> Nullable<Int4>,
        shopping_location_id -> Nullable<Int4>,
    }
}

table! {
    stock_log (id) {
        id -> Int4,
        product_id -> Int4,
        amount -> Float8,
        best_before_date -> Nullable<Date>,
        purchased_date -> Nullable<Date>,
        used_date -> Nullable<Date>,
        spoiled -> Bool,
        stock_id -> Text,
        transaction_type -> Text,
        price -> Nullable<Float8>,
        undone -> Bool,
        undone_timestamp -> Nullable<Timestamp>,
        opened_date -> Nullable<Timestamp>,
        row_created_timestamp -> Nullable<Timestamp>,
        location_id -> Nullable<Int4>,
        recipe_id -> Nullable<Int4>,
        correlation_id -> Nullable<Text>,
        transaction_id -> Nullable<Text>,
        stock_row_id -> Nullable<Int4>,
        shopping_location_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

table! {
    task_categories (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        done -> Bool,
        done_timestamp -> Nullable<Timestamp>,
        category_id -> Nullable<Int4>,
        assigned_to_user_id -> Nullable<Int4>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    user_permissions (id) {
        id -> Int4,
        permission_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Text,
        value -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
        row_updated_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    userentities (id) {
        id -> Int4,
        name -> Text,
        caption -> Text,
        description -> Nullable<Text>,
        show_in_sidebar_menu -> Bool,
        icon_css_class -> Nullable<Text>,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    userfield_values (id) {
        id -> Int4,
        field_id -> Int4,
        object_id -> Int4,
        value -> Text,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    userfields (id) {
        id -> Int4,
        entity -> Text,
        name -> Text,
        caption -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        show_as_column_in_tables -> Bool,
        row_created_timestamp -> Nullable<Timestamp>,
        config -> Nullable<Text>,
        sort_number -> Nullable<Int4>,
    }
}

table! {
    userobjects (id) {
        id -> Int4,
        userentity_id -> Int4,
        row_created_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        password -> Text,
        row_created_timestamp -> Nullable<Timestamp>,
        picture_file_name -> Nullable<Text>,
    }
}

joinable!(stock -> products (product_id));

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
