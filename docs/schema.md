# Common things

row_created_timestamp, undone, undone_timestamp, active

id, name, description

also that many tables have the table and a logging table

for types question mark means can be null.

all tinyints to booleans (for postgres)

enums in postgres

# chores

## assignment_config

type: text?

comma-separated list of user_id's that can potentially be assigned to this chore.

It seems like null doesn't mean everybody but doesn't work

who-least-did-first seems to not support this.

## next_execution_assigned_to_user_id

type: int?

the person who has to do this next

it seems like undo doesn't undo this?

## consume_product_on_execution

type: tinyint / bool (maybe some bools are nullable - then also fix that)

wether to consume a product on executing this chore

### improvements

merge into product_id - it being null means false

## product_id

type: tinyint (why tinyint?!?!?)

the product to consume on execution

### improvements

foreign key to products(id)

## product_amount

type: real

the amount of the product to consume on execution

### improvements

real is an interesting choice (is there not a "quantum" unit for everything?)

## period_interval

type: integer > 0 default 1

the amount of period_type

what is period_days then? look at https://github.com/mistressofjellyfish/not-grocy/blob/1f1c13dda13ec2a41c047bd16a078315ac9a97a5/js/viewjs/choreform.js and all the views to find out

## active

type: tinyint (bool)

whether this chore is active

# chores_log

## id

type: integer primary key

## chore_id

type: integer

### improvements

foreign key chore(id)

## tracked_time

type: datetime?

time when this was done

### improvements

non-null

## done_by_user_id

type: integer?

who did the chore

### improvements

foreign key user_id

maybe non-null - somebody must have done it

## row_created_timestamp

type: datetime? default now

## undone

type: tinyint (bool)

## undone_timestamp

type: datetime?

# equipment

some equipment mostly to associate it with an instruction manual.

may be unified with something else if descriptions can contain files (in a rich text field).

## id

type: integer primary key

## name

type: text unique

### improvements

unique really needed?

## description

type: text?

## instruction_manual_file_name

type: text?

file name of the instruction manual

what exactly does file name mean?

## row_created_timestamp

type: datetime? default now

# locations

## id

type: integer primary key

## name

type: text unique

## description

type: text?

## row_created_timestamp

type: datetime? default now

## is_freezer

type: tinyint (bool)

interesting - for what is this used?

I realize - food is complicated.

there are different best before dates depending on from where to where you move the food (I think it should only matter to where you move it)

default_best_before_days_after_freezing

default_best_before_days_after_thawing

still lot's of details missing here

# meal_plan

repetition planning problems seem similar like for chores / batteries but usually a meal plan is per week / different every day

## id

type: integer primary key

## day

type: date

### improvements

rename field to date?

## type

type: text? (recipe, product, note)

type of the entry in the meal plan

### improvements

crazy and probably stupid idea: why is a product not a recipe with no preparation?

## recipe_id

type: integer?

### improvements

foreign key to recipes?

## recipe_servings

type: integer?

number of servings of the recipe

## note

type: string?

a note (probably only valid when type==note).

### improvements

Generally here are three different types - I know relational databases don't like columns that depend on enums - maybe there is still a better way (e.g (remove type) and allow note always? possibly this is already the case)

## product_id

type: integer?

### improvements

foreign key to products

## product_amount

type: real? default 0

the amount of the product

interesting that the default here is 0.

### improvements

merge into recipe_servings?

## product_qu_id

type: integer?

probably quantity unit id

### improvements

rename to a more understandable name

foreign key to quantity_units(id)?

## row_created_timestamp

type: datetime? default now

# permission_hierarchy

hierarchy of permissions connected using their parents

https://github.com/mistressofjellyfish/not-grocy/blob/246c7fcb64d63caf1a4a1e8dafb0308d9fa4264b/php/Services/UsersService.php#L19

## id

type: integer primary key

## name

type: text unique

e.g. ADMIN

### improvements

check if id could be merged into name

## parent

type: integer?

the parent of this permission or null

### improvements

foreign key to permission_hierarchy(id)

from the permissions in the table this seems to be not super worth it although creating an admin is significantly easier in this way

# product_barcodes

# product_groups

# products

# quantity_unit_conversions

# quantity_units

# recipes

# recipes_nestings

# recipes_pos

# sessions

these are the active sessions of the users

## id

## session_key

## user_id

## expires

## last_used

## row_created_timestamp

# shopping_list

# shopping_lists

# shopping_locations

# stock

# stock_log

# task_categories

# tasks

# user_permissions

the permissions of a user

## id

### improvements

remove? and make the other two fields composed primary key

## permissions_id

### improvements

foreign key of permission_hierarchy(id)

## user_id

### improvements

foreign key of user(id)

# user_settings

# userentities

userentities are in the master data

they are shown in the navigation as an entry

as far as I understand them they're just custom tables

they reference multiple user fields

## id

## name

## caption

## description

## show_in_sidebar_menu

## icon_css_class

## row_created_timestamp

# userfield_values

this probably contains the values of these custom tables

## id

## field_id

## object_id

## value

## row_created_timestamp

# userfields

this is probably the list of columns for a user entity

## id

## entity

## name

## caption

## type

## show_as_column_in_tables

### improvements

this is probably the foreign key userentities(id)

## row_created_timestamp

## config

## sort_number

# userobjects

where is this in the hierarchy?

don't understand where userobjects and userfields belongs

maybe it's something completely different

## id

## userentity_id

probably foreign key userentities(id)

## row_created_timestamp
