cube = Box(1,1,1,0.3)
sphere = Sphere(0.5)
diff = Difference({cube, sphere}, 0.3)
diff=diff:scale(15,15,15)

build(diff)
