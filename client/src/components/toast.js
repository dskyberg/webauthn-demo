const showToast = (toast, status, message, title) => {
    toast({
        position: 'top',
        status: status,
        title: title,
        description: message,
        isClosable: true,
    })
}

export const setError = (toast, message) => {
    console.error(message)
    showToast(toast, 'error', message)
}

export const setWarning = (toast, message) => {
    console.warn(message)
    showToast(toast, 'warning', message)
}

export const setSuccess = (toast, message) => {
    console.log(message)
    showToast(toast, 'success', message)
}
