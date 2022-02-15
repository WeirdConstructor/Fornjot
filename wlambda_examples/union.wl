!@import fj;

!cube_a =
    fj:sweep
        (fj:sketch $[
            $f(-0.5, -0.5),
            $f( 0.5, -0.5),
            $f( 0.5,  0.5),
            $f(-0.5,  0.5)
        ])
        1.0;

fj:union
    (fj:trans cube_a $f(1.5, 0.0, 0.5))
    cube_a;
