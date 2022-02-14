!@import fj;

!spacer = {
    !outer  = fj:circle 1.0;
    !inner  = fj:circle 0.2;
    !height = 1.0;

    fj:sweep (fj:diff2d outer inner) height
};

!cuboid = {
    !x = 3.0;
    !y = 2.0;
    !z = 1.0;

    fj:sweep (fj:sketch $[
        $f(-(x / 2.0), -(y / 2.0)),
        $f( (x / 2.0), -(y / 2.0)),
        $f( (x / 2.0),  (y / 2.0)),
        $f(-(x / 2.0),  (y / 2.0)),
    ]) z;
};

cuboid[]
