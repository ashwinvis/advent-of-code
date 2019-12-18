module day01
use types, only: dp, pi

implicit none

contains

  integer elemental function fuel_from_mass(mod_mass) result(mod_fuel)
    ! find the fuel required for a module
    real(dp), intent(in) :: mod_mass
    ! take its mass, divide by three, round down, and subtract 2
    mod_fuel = nint(mod_mass / 3) - 2
  end function

  integer function total_fuel(masses) result(total)
    real(dp), intent(in) :: masses(:)
    integer :: fuels(size(masses))

    fuels = fuel_from_mass(masses)
    total = sum(fuels)
  end function

end module

