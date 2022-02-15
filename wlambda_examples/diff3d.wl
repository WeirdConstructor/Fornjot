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

fj:diff
    (fj:trans cube_a $f(0.5, 0.0, 0.5))
    cube_a;
