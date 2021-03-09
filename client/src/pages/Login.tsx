import React from 'react'
import LoginForm from "../components/LoginForm";

export default class Login extends React.Component {
    login() {
        console.log("Login!");
    }

    render() {
        return (
            <div>
                <LoginForm/>
            </div>
        )
    }
}