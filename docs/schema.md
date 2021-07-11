# Common things

row_created_timestamp, undone, undone_timestamp, active

id, name, description

also that many tables have the table and a logging table

for types question mark means can be null.

all tinyints to booleans (for postgres)

enums in postgres

bool (maybe some bools are nullable - then also fix that)

# recipes

# recipes_nestings

# recipes_pos

# shopping_list

# shopping_lists

# shopping_locations

# stock

# stock_log

# task_categories

# tasks

# user_settings

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
