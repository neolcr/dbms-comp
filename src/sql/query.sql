select * from tabla1 inner join tabla2 on tabla1.id = tabla2.id where tabla1.edad in (select edad from edades where id = 1) ;
