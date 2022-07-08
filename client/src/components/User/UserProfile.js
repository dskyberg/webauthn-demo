import { Text, Box, Grid, GridItem } from '@chakra-ui/react';

export default function UserProfile(props) {
    const { user } = props

    return (
        <Box mb="2rem" shadow="base"
            borderWidth="1px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" >
            <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={'yellow.500'}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'2'}
            >Profile</Text>
            <Grid templateColumns='repeat(5, 1fr)' spacing={2}>
                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Name </Text></GridItem>
                <GridItem colSpan={4}> <Text as={'span'} >{user.name}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Display Name </Text></GridItem>
                <GridItem colSpan={4}><Text as={'span'} >{user.displayName}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">ID </Text></GridItem>
                <GridItem colSpan={4}><Text as={'span'} >{user.id}</Text></GridItem>
            </Grid>
        </Box>
    )
}
