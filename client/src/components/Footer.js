import AppBar from '@mui/material/AppBar';
import Container from '@mui/material/Container';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';

import Link from '@mui/material/Link';

export default function Footer() {
    return (
        <AppBar position="static">
            <Container maxWidth="sm">
                <Toolbar>
                    <Typography variant="body2" color="text.secondary" align="center">
                        {'Copyright © '}
                        <Link color="inherit" href="https://mui.com/">
                            Your Website
                        </Link>{' '}
                        {new Date().getFullYear()}
                        {'.'}
                    </Typography>
                </Toolbar>
            </Container>
        </AppBar>
    );
}
