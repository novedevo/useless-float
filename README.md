# stupid floats

y'know how equality and ordering on floats are partial because ieee says nans aren't equal to anything? this fixes that by checking the random garbage in the nan's mantissas and pretending it means something. bonus this now allows for +nan and -nan. so nan == nan sometimes, only if they have the same bitpattern. nan > nan if the first is positive and the second is negative, or if they're the same sign and the first has a higher bit in the mantissa.

they're still partially ordered though because idk whether nans are greater or less than other numbers. make your own stupid float crate if you want that
