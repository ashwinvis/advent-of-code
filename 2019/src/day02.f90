module day02
implicit none

contains
  subroutine run_intcode(n, intcode)
    integer, intent(in) :: n
    integer, intent(inout) :: intcode(0:n-1)  ! 1D array with index starting as 0
    integer :: opcode, i0, i1, i2, i3

    do i0 = 0, n, 4
      opcode = intcode(i0)

      if (opcode == 99) exit

      ! Indices to fetch values from
      i1 = intcode(i0 + 1)
      i2 = intcode(i0 + 2)
      i3 = intcode(i0 + 3)

      ! Mutate intcode based on opcode
      if(opcode == 1) then
        intcode(i3) = intcode(i1) + intcode(i2)
      else if (opcode == 2) then
        intcode(i3) = intcode(i1) * intcode(i2)
      else
        stop "Illegal opcode!"
      end if
    end do

    print *, "End of intcode program"

  end subroutine
end module
