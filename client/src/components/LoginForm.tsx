import React from 'react';
import {Button, Input, Label} from "@windmill/react-ui";
import {Link} from "react-router-dom";

interface IState {
  email: string,
  password: string,
  remember: boolean
}

export default class LoginForm extends React.Component<any, IState> {
  constructor(props: any) {
    super(props);

    this.state = {
      email: '',
      password: '',
      remember: false
    }
  }


  login() {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id: this.state.email, password: this.state.password }),
    };
    fetch('/auth/login', requestOptions)
        .then((response) => response.json())
        .then((data) => console.log(data));
  }

  render() {
    return (
        <div className="flex items-center min-h-screen p-6 bg-gray-50 dark:bg-gray-900">
          <div className="flex-1 h-full max-w-4xl mx-auto overflow-hidden bg-white rounded-lg shadow-xl dark:bg-gray-800">
            <div className="flex flex-col overflow-y-auto md:flex-row">
              <div className="h-32 md:h-auto md:w-1/2">
              </div>
              <main className="flex items-center justify-center p-6 sm:p-12 md:w-1/2">
                <div className="w-full">
                  <h1 className="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Login</h1>
                  <Label>
                    <span>Email</span>
                    {/*@ts-ignore*/}
                    <Input className="mt-1" type="email" placeholder="john@doe.com" required={true}/>
                  </Label>

                  <Label className="mt-4">
                    <span>Password</span>
                    {/*@ts-ignore*/}
                    <Input className="mt-1" type="password" placeholder="***************" required={true}/>
                  </Label>
                  {/*@ts-ignore*/}
                  <Button className="mt-4" onClick={() => this.login()}>
                    Log in
                  </Button>

                  <hr className="my-8" />

                  <p className="mt-4">
                    <Link
                        className="text-sm font-medium text-purple-600 dark:text-purple-400 hover:underline"
                        to="/forgot-password"
                    >
                      Forgot your password?
                    </Link>
                  </p>
                  <p className="mt-1">
                    <Link
                        className="text-sm font-medium text-purple-600 dark:text-purple-400 hover:underline"
                        to="/create-account"
                    >
                      Create account
                    </Link>
                  </p>
                </div>
              </main>
            </div>
          </div>
        </div>
    );
  }
}
