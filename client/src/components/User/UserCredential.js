import { Text, Box, Grid, GridItem, HStack, Badge } from '@chakra-ui/react';

const USER_PRESENT = 1;
const USER_VERIFIED = 4;
const ATTESTED_CREDENTIAL_DATA_INCLUDED = 64;
const EXTENSION_DATA_INCLUDED = 128;


export default function UserCredential(props) {
    const { credential } = props

    if (credential === null) return (<></>)

    return (
        <Box m="2.0rm" shadow="base"
            borderWidth="1px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" >
            <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={'yellow.500'}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'2'}
            >Credential</Text>
            <Grid templateColumns='repeat(5, 1fr)' spacing={2}>
                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Type</Text></GridItem>
                <GridItem colSpan={4}><Text>{credential.type}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">AAGUID:</Text> </GridItem>
                <GridItem colSpan={4}><Text>{credential.aaguid}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">ID:</Text></GridItem>
                <GridItem colSpan={4}><Text>{credential.id}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Algorithm</Text></GridItem>
                <GridItem colSpan={4}><Alg alg={credential.credential_public_key.alg} /></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Key</Text> </GridItem>
                <GridItem colSpan={4}><Text>{credential.credential_public_key.bytes}</Text></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Flags</Text> </GridItem>
                <GridItem colSpan={4}><Flags flags={credential.flags} /></GridItem>

                <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Last Used</Text> </GridItem>
                <GridItem colSpan={4}><Text>{credential.last}</Text></GridItem>
            </Grid>
        </Box>
    )
}


const Alg = ({ alg }) => {
    let txt
    switch (alg) {
        case -7: txt = 'ES256'; break
        case -35: txt = 'ES384'; break
        case -8: txt = 'EDDSA'; break
        default: txt = `Unknown: ${alg}`;
    }
    return (
        <Text>{txt}</Text>
    )
}

const Flags = ({ flags }) => {
    const userPresent = (flags & USER_PRESENT) > 0
    const userVerified = (flags & USER_VERIFIED) > 0
    const acData = (flags & ATTESTED_CREDENTIAL_DATA_INCLUDED) > 0
    const extData = (flags & EXTENSION_DATA_INCLUDED) > 0
    let bits = flags.toString(2)
    if (flags < 129) {
        bits = '0' + bits
    }

    return (
        <HStack>
            {userPresent && <Badge colorScheme={'green'}>User Present</Badge>}
            {userVerified && <Badge colorScheme={'green'}>User Verified</Badge>}
            {acData && <Badge colorScheme={'teal'}>Attested</Badge>}
            {extData && <Badge colorScheme={'teal'}>Extensions</Badge>}
            <Badge>{bits}</Badge>
        </HStack>
    )
}


