import { Card, CardContent, Grid, GridItem } from '@chakra-ui/react';


export default function AuthenticatorCard(props) {
    const { authenticator } = props

    const toHex = (array) => array.map((x) => x.toString(16)).join(' ');

    const splitEvery = (value, len) => {
        const regex = new RegExp(`.{1,${len}}`, 'g')
        return value.match(regex).join('\n')
    }

    return (
        <Card >
            <CardContent>
                <Grid >
                    <GridItem>{splitEvery(authenticator.credID, 50)}</GridItem>
                    <GridItem>{authenticator.fmt}</GridItem>
                    <GridItem>{authenticator.counter}</GridItem>
                    <GridItem>{splitEvery(authenticator.publicKey, 50)}</GridItem>
                    <GridItem>{authenticator.attestation.format}</GridItem>
                    <GridItem>{authenticator.attestation.alg}</GridItem>
                    <GridItem>{toHex(authenticator.aaguid.data)}</GridItem>
                </Grid>
            </CardContent >
        </Card >
    )
}
