import Button from '@mui/material/Button'
import Menu from '@mui/material/Menu'
import MenuItem from '@mui/material/MenuItem'
import { useRouter } from 'next/router'
import { useState } from 'react'






export default function BasicMenu() {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null)
  const open = Boolean(anchorEl)

  const router = useRouter();

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget)
  }

  const handleClose = () => {
    setAnchorEl(null)
  }

  const routeToPage = (url: string) => {  
    router.push(url)
  }

  return (
    <div className="md:!hidden">
      <Button
        id="basic-button"
        aria-controls={open ? 'basic-menu' : undefined}
        aria-haspopup="true"
        aria-expanded={open ? 'true' : undefined}
        onClick={handleClick}
        className="!capitalize !text-white"
      >
        Browse
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
        <MenuItem onClick={() => {routeToPage("/")}}>Home</MenuItem>
        <MenuItem onClick={() => {routeToPage("/")}}>TV Shows</MenuItem>
        <MenuItem onClick={() => {routeToPage("/")}}>Movies</MenuItem>
        <MenuItem onClick={() => {routeToPage("/")}}>New & Popular</MenuItem>
        <MenuItem onClick={() => {routeToPage("/")}}>My List</MenuItem>
      </Menu>
    </div>
  )
}