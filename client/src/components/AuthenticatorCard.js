/** @jsxImportSource @emotion/react */
import { Fragment } from 'react'
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography';

function Row(props) {
    const { label, value } = props
    return (
        <Fragment>
            <Grid item xs={2}>
                <Typography variant="body2"><strong>{label}</strong></Typography>
            </Grid>
            <Grid item xs={10} component="div">
                <Typography variant="body2" component="div" noWrap={false}>{value}</Typography>
            </Grid>
        </Fragment>
    )
}

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
                <Grid container rowSpacing={2} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
                    <Row label="ID" value={splitEvery(authenticator.credID, 50)} />
                    <Row label="Format" value={authenticator.fmt} />
                    <Row label="Counter" value={authenticator.counter} />
                    <Row label="Public key" value={splitEvery(authenticator.publicKey, 50)} />
                    <Row label="Att. Format" value={authenticator.attestation.format} />
                    <Row label="Att. Alg" value={authenticator.attestation.alg} />
                    <Row label="AAGUID" value={toHex(authenticator.aaguid.data)} />
                </Grid>
            </CardContent >
        </Card >
    )
}
