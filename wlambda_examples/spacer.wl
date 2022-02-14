!@import fj;

!spacer = {
    !outer  = fj:circle 1.0;
    !inner  = fj:circle 0.2;
    !height = 1.0;

    fj:sweep (fj:diff2d outer inner) height
};

spacer[]
