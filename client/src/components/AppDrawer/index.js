import { useNavigate } from 'react-router-dom'
import { Drawer, DrawerOverlay, DrawerContent, Divider, DrawerBody, Stack, DrawerCloseButton, Button } from '@chakra-ui/react'
import { TbSettings } from 'react-icons/tb'
import { HiHome } from 'react-icons/hi'
import { FaRegAddressCard } from 'react-icons/fa'
import { RiLoginBoxLine } from 'react-icons/ri'


export default function AppDrawer(props) {
    const { btnRef, isOpen, onClose, onSettingsOpen } = props
    const navigate = useNavigate()

    const handleLink = (event) => {
        onClose()
        navigate(`${event.target.id}`)
    }

    const handleSettingsClicked = event => {
        onClose()
        onSettingsOpen(event)
    }
    return (
        <Drawer
            isOpen={isOpen}
            placement="left"
            onClose={onClose}
            finalFocusRef={btnRef}
        >
            <DrawerOverlay />
            <DrawerContent>
                <DrawerCloseButton />
                <DrawerBody>
                    <Stack direction="column" align={"flex-start"}>
                        <Button leftIcon={<HiHome />} id="/" variant="ghost" onClick={handleLink}>Home</Button>
                        <Button leftIcon={<FaRegAddressCard />} id="/register" variant="ghost" onClick={handleLink}>Register</Button>
                        <Button leftIcon={<RiLoginBoxLine />} id="/login" variant="ghost" onClick={handleLink}>Login</Button>
                        <Button leftIcon={<TbSettings />} variant="ghost" onClick={handleSettingsClicked}>Settings</Button>
                        <Divider />
                        <Button leftIcon={<RiLoginBoxLine />} id="/users" variant="ghost" onClick={handleLink}>Users</Button>

                    </Stack>
                </DrawerBody>

            </DrawerContent>
        </Drawer>

    )
}