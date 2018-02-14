# Category

字段

```sql
id， name, desc, super
```

## App : Searchable, Rankable, Starable, Praisable

字段

```sql
id, graph_flag, parent, super, creator, pkgname, name, alias, applayout, optbtn, blame, installurl, removeurl, apimin, apitar, size, version, reversion, special, desc, updates, lang, srcurl, homeurl, license, perm, dev, root_stat, need_touch, need_framework, created_at, updated_at, stars, count, rank, rank_avg, replies, pinned_messages
```

## App Update

字段

```sql
target, graph_flag, name, alias, optbtn, blame, installurl, removeurl, apimin, apitar, size, version, reversion, updates, perm, released_at
```

## Recommend

字段

```sql
target recommend uid reason created_at
```

## Follow

字段

```sql
uid target created_at
```

## User : Searchable

字段

```sql
id, name, alias, email, bio, github, superapp, created_at, online_at
```

## User Hash

字段

```sql
target, hash
```

## App Last cid

字段

```sql
target, last
```

## Comment : Searchable, Starable, Praisable

字段

```sql
id sender target cid reply_to text created_at updated_at stars count replies is_pop
```

## Headline : Searchable

字段

```sql
graph_flag, badge, label, link, created_at, open
```

## Post

字段

```sql
from, target, type, cmid, created_at
```

## PM

字段

```sql
id, user, type, accessable_to, text, created_at, updated_at
```

## Message Record

字段

```sql
user, cmid, created_at
```

## App Star

字段

```sql
target user
```

## Comment Star

字段

```sql
target user
```

## App Praise

字段

```sql
target user
```

## Comment Praise

字段

```sql
target user
```

## Rank

字段

```sql
target user rank
```
