# GeekApkR Server APIs

## Category

> Get Category Tree

`GET /category` -> categories_tree

> Get Toplevel Categories

`GET /category/top` -> JSON { [category_id] }

> Get All Categories

`GET /category/all` -> JSON { [category_id] }

> Read a category

`GET /category/<id>` -> JSON { name, desc, super, childs }

> Read a category attribute

`GET /cateogry/<id>/<attr>` -> attr

> New category

`GET /category/new/<id>/uid=<uid>&tok=<token>&name=<name>&desc=<desc>&super=<super>` -> JSON { status, reason }

> Edit category

`PUT /category/<id>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Delete category

`DELETE /category/<id>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Comments

> Add Comment

`POST /comment/<target>?uid=<uid>&tok=<token>&reply=<reply_to>` body=text -> JSON { status, reason, cmid, cid, is_pop }

> Get Comment

`GET /comment/<target>/<cid>` -> JSON { cmid, status, reason }

`GET /comment/<cmid>` -> JSON { target, cid, sender, text, reply_to, childs, created_at, updated_at, stars, count, replies, is_pop }

> Get Attr

`GET /commment/<cmid>/<attr>` -> attr

> Update

`PUT /comment/<cmid>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Delete Comment

`DELETE /comment/delete/<cmid>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> All Comments

`GET /comment/len/<all|pops|target>` -> num

`GET /comment/all/<start>-<end>?filter=<filter>&[r]sort=<sort>[&toplev]` -> JSON { [id] }

`GET /comment/pops/<start>-<end>?filter=<filter>&[r]sort=<sort>[&toplev]` -> JSON { [id] }

`GET /comment/target/<target>/<start>-<end>?filter=<filter>&[r]sort=<sort>[&toplev]` -> JSON { [id] }

`GET /comment/r/<cmid>[?noextra]` -> JSON { [{id, sender, text, replies_num}] }

filter -> user userlist

sort -> star count replies created updated

> Search

`POST /comment/search?[inapp=<aid>&]filter=<filter>&[r]sort=<sort>[&toplev]` body=text -> JSON { [id] }

`POST /comment/searchpops?[inapp=<aid>&]filter=<filter>&[r]sort=<sort>[&toplev]` body=text -> JSON { [id] }

## User

> Get Data

`GET /user/<uid>` -> JSON { name, alias, email, bio, github, superapp, created_at, online_at }

`GET /user/<uid>/<attr>` -> attr

> Update

`PUT /user/<uid>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Reomve

`DELETE /user/delete/<uid>?hash=<hash>&do=<type>` -> JSON { status, reason }

> All

`GET /user/all?[r]sort=<sort>&filter=<filter>`

> Search

`POST /user/search?[r]sort=<sort>&filter=<filter>`

filter -> superuser hasemail hasbio hasalias inlist

sort -> ctime online follow followed

> Image

`GET /img/<uid>` -> JSON { [{id, created_at}] }

`GET /img/<uid>/<id>` -> File

`POST /img/<uid>?tok=<token>` -> JSON { status, reason, id }

`DELETE /img/<uid>/<id>?tok=<token>` -> JSON { status, reason }

## App

> Create

`POST /app/create?uid=<uid>&tok=<token>` body=JSON { graph_flag, parent, super,  pkgname, name, alias, applayout, optbtn, installurl, removeurl, apimin, apitar, size, version, reversion, special, desc, updates, lang, srcurl, homeurl, license, perm, dev, root_stat, need_touch, need_framework, pinned_messages } -> JSON { status, reason, aid }

> Read

`GET /app/<aid>` -> JSON { graph_flag, parent, super, creator, pkgname, name, alias, applayout, optbtn, blame, installurl, removeurl, apimin, apitar, size, version, reversion, special, desc, updates, lang, srcurl, homeurl, license, perm, dev, root_stat, need_touch, need_framework, created_at, updated_at, stars, count, rank, rank_avg, replies, pinned_messages }

`GET /app/<aid>/<attr>` -> attr

`GET /app/all[?sort=<sort>&filter=<f>]` -> JSON { [id] }

> Update

`PUT /app/<aid>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Delete

`DELETE /app/<aid>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Search

`GET /app/search[?sort=<sort>&filter=<f>]` body=text -> JSON { [id] }

`GET /app/search/<uid>[?sort=<sort>&filter=<f>]` -> JSON { [id] }

`GET /app/search/category/<id>[?sort=<sort>&filter=<f>&mode=<mode>]` body=text -> JSON { [id] }

mode -> super super_super

sort -> star count replies rank rank_avg created updated size

filter -> apimin、updated、root、touch、framework、user

> Image

`GET /img/app/<aid>` -> JSON { [{rev, created_at}] }

`GET /img/app/<aid>/<rev>` -> File

`POST /img/app/<aid>/<rev>?tok=<token>` -> JSON { status, reason }

`DELETE /img/app/<uid>/<rev>?tok=<token>` -> JSON { status, reason }

> Preview

`GET /img/prev/<aid>` -> JSON { [{slots}] }

`GET /img/prev/<aid>/<slot#>` -> File

`POST /img/prev/<aid>/<slot#>?tok=<token>` -> JSON { status, reason }

`DELETE /img/prev/<aid>/<slot#>?tok=<token>` -> JSON { status, reason }

## App Update

> Create

`POST /update/<aid>?uid=<uid>&tok=<token>` body=JSON { graph_flag, name, alias, optbtn, installurl, removeurl, apimin, apitar, size, version, reversion, updates, perm } -> JSON { reversion, status, reason }

> Read

`GET /update/<aid>/` -> JSON { [rev] }

`GET /update/<aid>/<rev>` -> Data

`GET /update/<aid>/<rev>/<attr>` -> attr

> Update

`PUT /update/<aid>/<rev>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Delete

`DELETE /update/<aid>/<rev>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Recommend

> Create

`POST /recommend/<aid>?uid=<uid>&tok=<tok>&rel=<rel>` body=reason -> JSON { status, reason }

> Read

`GET /recommend/<aid>` -> JSON { [recommends] }

`GET /recommend/user/<uid>` -> JSON { [recommends] }

`GET /recommend/app/<aid>` -> JSON { [recommends] }

> Update

`PUT /recommend/<aid>/<ctime>/<attr>?uid=<uid>&tok=<token>` body=value -> JSON { status, reason }

> Delete

`DELETE /recommend/<aid>/<ctime>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Follow

> Create

`GET /follow/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Read

`GET /follow/<target>` -> JSON { followers, following }

`GET /follow/<target>/<attr>` -> JSON { [id] }

`GET /follow/followed?a=<a>&b=<b>` -> followed

> Delete

`DELETE /follow/<target>?uid=<id>&tok=<token>` -> JSON { status, reason }

## Headline

> Create

`POST /headline/new?uid=<uid>&tok=<token>` body=JSON { graph_flag, badge, label, link, id }

> Read

`GET /headline/all` -> ...

`GET /headline/open` -> ...

`GET /headline/<id>` -> ...

`GET /headline/<id>/<attr>` -> ...

> Update

`PUT /headline/<id>/<attr>?uid=<uid>&tok=<token>` body=val -> ...

`PUT /headline/<id>?uid=<uid>&tok=<token>` -> ...

> Delete

`DELETE /headline/<id>?uid=<uid>&tok=<token>` -> ...

> Search

`POST /headline/search` body=text -> JSON { [ids] }

> Image

`GET /img/headline/<id>` -> File

`POST /img/headline/<id>?uid=<uid>&tok=<token>` -> JSON { status, reason }

`DELETE  /img/headline/<id>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Post

> Create

`GET /post/<target>?uid=<uid>&tok<token>&type=<type>&ext=<ext>` -> JSON { status, reason }

> Read

`GET /post/me?uid=<uid>&tok=<token>[&type=<type>]` -> JSON { [post] }

> Delete

`DELETE /post/<created_at>?uid=<uid>&tok=<token>`

## PM

> Create

`POST /pm/<uid>?tok=<token>&type=<type>&access=<accessable_to>` body=text -> JSON { id, status, reason }

> Read

`GET /pm/id/<id>?uid=<uid>&tok=<token>` -> JSON { user, type, accessable_to, text, created_at, updated_at }

`GET /pm/mine?uid=<uid>&tok=<token>[&type=<type>&access=<access_to>]` -> JSON { [pm] }

`GET /pm/shared?uid=<uid>&tok=<token>[&sender=<sender>&type=<type>&access=<access_to>]` -> JSON { [pm] }

> Update

`PUT /pm/id/<id>/<attr>` body=value -> JSON { status, reason }

> DELETE

`DELETE /pm/id/<id>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Message Record

> Read

`GET /mrc/<uid>` ident -> record num

`GET /mrc/<uid>/[<start>-<end>]?[sort=r&]tok=<token>` -> JSON { [{cmid, created_at}] }

> DELETE

`DELETE /mrc/<uid>?created=<created_at>&tok=<token>` -> JSON { status, reason }

`DELETE /mrc/<uid>?created_to=<created_at>&tok=<token>` -> JSON { [cmid], status, reason }

## App Star

> Create

`GET /star/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Read

`GET /star/<target>` -> JSON { [uid] }

`GET /star/uid/<uid>` -> JSON { [aid] }

> DELETE

`DELETE /star/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Comment Star

> Create

`GET /cstar/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Read

`GET /cstar/<target>` -> JSON { [uid] }

`GET /cstar/uid/<uid>` -> JSON { [cmid] }

> DELETE

`DELETE /cstar/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## App Praise

> Create

`GET /priase/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Read

`GET /praise/<target>` -> JSON { [uid] }

`GET /praise/uid/<uid>` -> JSON { [aid] }

> DELETE

`DELETE /praise/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Comment Praise

> Create

`GET /cpraise/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Read

`GET /cpraise/<target>` -> JSON { [uid] }

`GET /cpraise/uid/<uid>` -> JSON { [aid] }

> DELETE

`DELETE /cpraise/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Rank

> Create

`GET /rank/<target>?uid=<uid>&tok=<token>&rank=<rank>` -> JSON { status, reason }

> Read

`GET /rank/<target>` -> JSON { [uid, rank] }

`GET /rank/uid/<uid>` -> JSON { [aid, rank] }

> DELETE

`DELETE /rank/<target>?uid=<uid>&tok=<token>` -> JSON { status, reason }

## Meta

> Meta Information

`GET /meta/info` -> JSON { name, desc, authors }

`GET /meta/special` -> JSON { app_news, cate_albums, cate_topics }

> Server Version

`GET /meta/version` -> JSON { version, reversion, codename }

> User Register

`GET /meta/register?auth=<auth>&gist=<gistid>&passwd=<inithash>` -> JSON { status, reason, uid }

> New User

`GET /meta/newuser?uid=<uid>&tok=<token>&passwd=<inthash>` -> JSON { status, reason, uid }

> User Reset Password

`GET /meta/reset?gist=<gistid>&auth=<auth>&passwd=<newhash>` -> JSON { status, reason, uid }

> Check Hash

`GET /meta/hashfor?uid=<uid>&hash=<hash>` -> JSON { status, reason }

> New Token

`GET /meta/retoken?uid=<uid>&hash=<hash>&tok=<token>` -> JSON { status, reason }

> Check Token

`GET /meta/tokenfor?uid=<uid>&tok=<token>` -> JSON { status, reason }

> Add blame

`POST /meta/blame/<aid>[/<rev>]?uid=<uid>&tok=<token>` body=text -> JSON { status, reason }

## Socket

> Send /uid ':' token/ to get private notification

+ HEADLINE
+ POST
+ NEWAPP
+ NEWUPD
+ NEWTOPIC
+ NEWS
+ NEWRECORD
