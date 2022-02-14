!@import fj;

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
