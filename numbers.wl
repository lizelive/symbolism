add[] = 0

add[a_Real, b_Real] = Native`Number`RealAdd[a, b]
add[a_Integer, a_Integer] = Native`Number`RealAdd[a, b]
add[b_Real, a_Integer] = Native`Number`RealAdd[a, b]



Construct[f_, x___] = f[x]

SetDelayed[apply[Patern[f,Blank[]]], Function[f[SlotSequance[1]]]]