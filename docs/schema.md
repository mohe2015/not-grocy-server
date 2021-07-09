# Common things

row_created_timestamp, undone, undone_timestamp

also that many tables have the table and a logging table


# api_keys

https://github.com/mistressofjellyfish/not-grocy/blob/vuejs/php/Services/ApiKeyService.php

In the current implementation it seems (GetOrCreateApiKey) like they get reused for different users / are not associated with a user. If this actually happens reusing is not a good idea as you can't revoke single api keys from a specific usage then.
https://github.com/mistressofjellyfish/not-grocy/blob/246c7fcb64d63caf1a4a1e8dafb0308d9fa4264b/php/Controllers/CalendarApiController.php#L57 they're reused for the ical sharing link so you can't revoke access per person you gave the link to. This may not be that important as it's a household management software but still could be better.

## id

type: integer

primary key

### improvements

remove and make api_key the primary key

## api_key

type: text

unique

the api key to authenticate with.

## user_id

type: integer

references a row in users table?

### improvements

make foreign key to users?

## expires

type: datetime?

when the api key expires. By default this is set to a date in 2999.

### improvements

could be non-null

## last_used

type: datetime?

usage unknown

### improvements

could be non-null if we consider creation a use (which is questionable)

## row_created_timestamp

type: datetime? default now

the time this row was created (usage unknown)

### improvements

non-null

## key_type

type: text (one of "default", "special-purpose-calendar-ical")

probably what permissions the api key has

### improvements

possibly type enum if database support exists

could possibly be unified with the permission system of users - so an api key is a (sub)user or whatever. Maybe this may also be too complicated though.

# batteries

## id

type: integer

primary key

## name

type: text

unique

the name of the battery

### improvements

not sure if uniqueness needs to be enforced - should not matter

## description

type: text?

the description of the battery

## used_in

type: text?

where the battery is used in

## charge_interval_days

type: integer default 0

the interval you want to charge the battery at in days

### improvements

depending on what 0 is supposed to mean maybe use null? As in I want this battery tracked when I charge it but I don't have a specific charge cycle.

## row_created_timestamp

type: datetime default now

## active

type: tinyint

whether the battery is still in use

### improvements

probably an sqlite thing but is a bool

# battery_charge_cycles

## id

type: integer primary key

## battery_id

type: text (wtf)

the battery this charge cycle belongs to

### improvements

should be type integer and foreign key battery(id)

## tracked_time

type: datetime?

the time at which you charged the battery

### improvements

non-null.

## row_created_timestamp

type: datetime? default now

### improvements

non-null (probably also for the others).

## undone

type: tinyint (bool)

whether this charge cycle was undone

### improvements

think about merging into undone_timestamp and use null instead? / use no timestamp?

## undone_timestamp

type: datetime?

when it was undone

# chores

# chores_log

# equipment

# locations

# meal_plan

# permission_hierarchy

# product_barcodes

# product_groups

# products

# quantity_unit_conversions

# quantity_units

# recipes

# recipes_nestings

# recipes_pos

# sessions

# shopping_list

# shopping_lists

# shopping_locations

# stock

# stock_log

# task_categories

# tasks

# user_permissions

# user_settings

# userentities

# userfield_values

# userfields

# userobjects

# users