import Button from '@mui/material/Button'
import Menu from '@mui/material/Menu'
import MenuItem from '@mui/material/MenuItem'
import { useRouter } from 'next/router'
import { useState } from 'react'

interface Props { 
  sort_by_popular: () => void,
  sort_by_least_popular: () => void,
  set_total_result: (value: React.SetStateAction<number | null>) => void
}


export default function FilterMenu({sort_by_popular, sort_by_least_popular, set_total_result}: Props) {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null)
  const open = Boolean(anchorEl)

  const router = useRouter();

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget)
  }

  const handleClose = () => {
    setAnchorEl(null)
  }

  return (
    <div className="">
      <Button
        id="basic-button"
        aria-controls={open ? 'basic-menu' : undefined}
        aria-haspopup="true"
        aria-expanded={open ? 'true' : undefined}
        onClick={handleClick}
        className="!capitalize !text-white"
      >
        <h2 className='font-semibold text-xl p-4  rounded-md bg-white text-black '>Popular</h2>
      </Button>
      <Menu
        id="basic-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        className="menu"
        MenuListProps={{
          'aria-labelledby': 'basic-button',
        }}
      >
        <MenuItem onClick={sort_by_popular}>Most Popular</MenuItem>
        <MenuItem onClick={sort_by_least_popular}>Least Popular</MenuItem>
        <MenuItem onClick={() => {}}>Companies You Follow (just teasing)</MenuItem>
        <MenuItem onClick={() => {}}>My List</MenuItem>
        <MenuItem >
          <input 
                      className='text-black scrollbar-none bg-gray-100 flex p-1 pb-5 scrollbar-hide font-normal text-md'
                      type="number" 
                      placeholder='Specify the number of search results' 
                      onChange={e => set_total_result(+e.target.value)}/>
          </MenuItem>
      </Menu>
    </div>
  )
}