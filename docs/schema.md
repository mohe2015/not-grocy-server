# Common things

row_created_timestamp, undone, undone_timestamp, active

id, name, description

also that many tables have the table and a logging table

for types question mark means can be null.

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

for the non-native speakers: [a job or piece of work that is often boring or unpleasant but needs to be done regularly](https://dictionary.cambridge.org/dictionary/english/chore)

couldn't the battery handling be merged into this?

maybe look at a calendar implementation to find a proper way to implement the recurring things here - maybe another relation is needed for the repetitions - so one chore can be repeated multiple times at different intervals. maybe we could just use some calendar implementation?

so my calendar implementation is as follows:

repetition_type: never, day, week, month, year
repetition_amount: number of that when to repeat
for week: weekday
for month: day of month or (first/.../last/second-last weekday) but this is probably overkill
for year: month and (first/.../last/second-last weekday) of that month

## id

type: integer primary key

## name

type: text unique

### improvements

really needs to be unique?

## description

type: text?

## period_type

type: text

seems to be one of manually, dynamic-regular, daily, weekly, monthly, yearly

https://github.com/mistressofjellyfish/not-grocy/blob/vuejs/php/Services/ChoresService.php#L12

### improvements

think about using an enum (not supported by sqlite)/integer.

## period_days

type: integer?

relation to period_type not exactly understood yet

## row_created_timestamp

type: datetime? default now

## period_config

type: text?

probably: comma separated list of weekdays if period_type == weekly

## track_date_only

type: tinyint (bool)

whether to only track the date

### improvements

Is this even needed?

## rollover

type: tinyint (bool)

"- New option "Due date rollover" per chore which means the chore can never be overdue, the due date will shift forward each day when due"

### improvements

Is this really needed? What's the intention? (I really wanna know - is it like mow the lawn, well we can just ignore that nobody cares)

## assignment_type

type: text?

one of in-alphabetical-order, no-assignment, random, who-least-did-first

decides who's next doing this chore

random means a random user of the assigned users is chosen

no-assignment means nobody gets assigned

in-alphabetical-order means the next one in the list of assigned users is chosen based on alphabetical order of the names.

who-least-did-first spies the history and gets the person who did it the least times in total past history. details in view chores_execution_users_statistics

https://github.com/mistressofjellyfish/not-grocy/blob/vuejs/php/Services/ChoresService.php#L7

### Improvements

for in-alphabetical-order why not simply choose the next one in the list, so not alphabetical (maybe for "determinism"?)

for who-least-did-first - this probably doesn't work if you get added later

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