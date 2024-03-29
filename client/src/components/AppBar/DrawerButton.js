import { IconButton } from '@chakra-ui/react';
import { GoThreeBars } from "react-icons/go";

export default function DrawerButton(props) {
    const { btnRef, onBtnClicked, ...rest } = props


    const handleClick = () => {
        onBtnClicked()
    }

    return (
        <IconButton
            ref={btnRef}
            size="sm"
            fontSize="lg"
            variant="ghost"
            color="current"
            marginLeft="2"
            icon={<GoThreeBars />}
            onClick={handleClick}
            {...rest}
        />
    )
}
