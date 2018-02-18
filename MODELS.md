# Models

> 模型关系 ~~许多都是胡扯，这不算是个好文档 只供参考~~

## Data

### Category

字段

```sql
id， name, desc, super
```

关系

__category__ : 1:N 一个分类可以有许多个子分类

### App : Searchable, Rankable, Starable, Praisable

字段

```sql
id, graph_flag, parent, super, creator, pkgname, name, alias, applayout, optbtn, blame, installurl, removeurl, apimin, apitar, size, version, reversion, special, desc, updates, lang, srcurl, homeurl, license, perm, dev, root_stat, need_touch, need_framework, created_at, updated_at, stars, count, rank, rank_avg, replies, pinned_messages
```

关系

__user__ : 1:1 一个应用只能有一个创建人

__category__ : 1:1 一个应用只能属于一个分类 一个应用只能属于一个超分类

__app update__ : 1:N 一个应用可以有许多版本

__recommend__ : 1:N 一个应用可以有许多推荐

__comment__ : 1:N 一个应用可以有许多评论

__app praise__ : 1:N 一个应用可以有许多点赞

__app star__ : 1:N 一个应用可以有许多 Star

### App Update

字段

```sql
target, graph_flag, name, alias, optbtn, blame, installurl, removeurl, apimin, apitar, size, version, reversion, updates, perm, released_at
```

__app__ : N:1 许多更新属于一个应用

### Recommend

字段

```sql
target, recommend, uid, reason, created_at
```

__app__ : N:1 许多推荐属于一个应用

__app__ : N:1 一个应用可以被推荐许多次

__user__ : N:1 一个用户有许多推荐

### Follow

字段

```sql
uid, target, created_at
```

关系

__user__ : 1:N 一个用户可以可以跟随许多用户

### User : Searchable

字段

```sql
id, name, alias, email, bio, github, superapp, created_at, online_at, followers, followed
```

关系

__follow__ : 1:N 一个用户可以有许多跟随

__app__ : 1:N 一个用户可以有许多应用

### User Hash

字段

```sql
target, hash
```

关系

__user__ : 1:1 一个用户对应一个 Hash

### App Last cid

字段

```sql
target, last
```

__app__ : 1:1

### Comment : Searchable, Starable, Praisable

字段

```sql
id, sender, target, cid, reply_to, text, created_at, updated_at, stars, count, replies, is_pop
```

__app__ : N:1

__user__ : N:1

### Headline : Searchable

字段

```sql
id, graph_flag, badge, label, link, created_at, open
```

### Post

字段

```sql
from, target, type, cmid, created_at
```

__user__ : N:1 许多 Post 属于一个用户

### PM

字段

```sql
id, user, type, accessable_to, text, created_at, updated_at
```

关系

__user__ : N:1 许多 PM 属于一个用户

### Message Record

字段

```sql
user, cmid, created_at
```

关系

__user__ : N:1 许多消息记录属于一个用户

### App Star

字段

```sql
target, user, created_at
```

关系

__app__ : N:1 许多 Star 属于一个应用

__user__ : N:1 一个用户有许多 Star

### Comment Star

字段

```sql
target, user, created_at
```

关系

__comment__ : N:1 许多 Star 属于一个评论

__user__ : N:1 一个用户有许多 Star

### App Praise

字段

```sql
target, user, created_at
```

关系

__app__ : N:1 许多赞属于一个应用

__user__ : N:1 一个用户有许多赞

### Comment Praise

字段

```sql
target, user, created_at
```

关系

__comment__ : N:1 许多赞属于一个评论

__user__ : N:1 一个用户有许多赞

### Rank

字段

```sql
target, user, rank, created_at
```

__app__ : N:1 许多评价属于一个应用

__user__ : N:1 一个用户有许多评价

## Runtime

### IP Based Rate Limiting

> HashMapping

```plain
ip => last_at, day
```

### Token Auth, Daily Limition

> HashMapping

```plain
user => day
```

```plain
user => token
```
