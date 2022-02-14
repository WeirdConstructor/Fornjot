!@import fj;

!outer  = fj:circle 1.0;
!inner  = fj:circle 0.2;
!height = 1.0;

!r = fj:sweep (fj:diff2d outer inner) height;

std:displayln r;

r
