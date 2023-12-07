select a, b
from hoge
left outer join fuga
on hoge.c = fuga.c
where hoge.d = 1
order by fuga.b

