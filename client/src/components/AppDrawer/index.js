import { useNavigate } from 'react-router-dom'
import { Drawer, DrawerOverlay, DrawerContent, DrawerHeader, DrawerBody, VStack, DrawerCloseButton, Button } from '@chakra-ui/react'

export default function AppDrawer(props) {
    const { btnRef, isOpen, onClose } = props
    const navigate = useNavigate()

    const handleLink = (event) => {
        onClose()
        navigate(`${event.target.id}`)
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
                <DrawerHeader>Pages</DrawerHeader>

                <DrawerBody>
                    <VStack>
                        <Button id="/" variant="ghost" onClick={handleLink}>Home</Button>
                    </VStack>
                </DrawerBody>

            </DrawerContent>
        </Drawer>

    )
}