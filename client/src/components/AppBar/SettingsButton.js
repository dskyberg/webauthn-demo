import { IconButton } from '@chakra-ui/react';
import { TbSettings } from 'react-icons/tb'

export default function DrawerButton(props) {
    const { btnRef, onClick, ...rest } = props
    return (
        <IconButton
            ref={btnRef}
            size="sm"
            fontSize="lg"
            variant="ghost"
            color="current"
            marginLeft="2"
            icon={<TbSettings />}
            onClick={onClick}
            {...rest}
        />
    )
}
