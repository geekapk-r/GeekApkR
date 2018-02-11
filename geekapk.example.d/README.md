# GeekApk Server Example Initialization Data

GeekApk Server needs something like initial categories/apps, initial users

Here is one used by geekapk.org GeekApk Server

## example server config

GeekApk Server searches for server config([geekapk.ini](geekapk.ini)) in its startup dir.

## internal picture storage

+ [avatars](geekapk_images/avatars/) uploaded avatars
+ [headpics](geekapk_images/headpics/) picture headlines
+ [icons](geekapk_images/icons/) application icons
+ [previews](geekapk_images/previews/) application previews

## database initialization files

+ [geekapk_main](dbinit.d/geekapk_main.sql) main tables
+ [geekapk_comments](dbinit.d/geekapk_comments.sql) comment tables
+ [geekapk_secure](dbinit.d/geekapk_secure.sql) stores user passwords and private tables
+ [geekapk_pops](dbinit.d/geekapk_pops.sql) "friendship cricle"