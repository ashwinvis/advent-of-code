module day01

implicit none

contains

  integer elemental function fuel_from_mass(mod_mass) result(mod_fuel)
    ! find the fuel required for a module
    use types, only: dp
    real(dp), intent(in) :: mod_mass
    ! take its mass, divide by three, round down, and subtract 2
    mod_fuel = floor(mod_mass / 3) - 2
  end function

  integer function total_fuel(masses) result(total)
    use types, only: dp
    real(dp), intent(in) :: masses(:)
    integer :: fuels(size(masses))

    fuels = fuel_from_mass(masses)
    total = sum(fuels)
  end function

  integer elemental function fuel_from_mass_rec(mod_mass) result(rec_fuel)
    ! recursively find the fuel required for a module
    use types, only: dp
    real(dp), intent(in) :: mod_mass
    real(dp) :: tmp_mass
    real(dp) :: mod_fuel

    tmp_mass = mod_mass
    mod_fuel = 0._dp
    rec_fuel = 0
    do while(mod_fuel .ge. 0)
      rec_fuel = rec_fuel + mod_fuel
      mod_fuel = floor(tmp_mass / 3) - 2
      tmp_mass = mod_fuel
    end do

  end function

  integer function total_fuel_rec(masses) result(total)
    use types, only: dp
    real(dp), intent(in) :: masses(:)
    integer :: fuels(size(masses))

    fuels = fuel_from_mass_rec(masses)
    total = sum(fuels)
  end function

end module

