import React, { useState } from 'react'
import Head from 'next/head'
import Image from 'next/image'
import Link from 'next/link'
import { SubmitHandler, useForm } from 'react-hook-form'
import useAuth from '../hooks/useAuth'


interface LoginInput { 
  email: string,
  password: string
}

function Login() {
    const [login, setlogin] = useState(false);
    const { register, handleSubmit, watch, formState: { errors } 
      } = useForm<LoginInput>();
    
    // Authenticate the User
    const { sign_in, sign_up } = useAuth();
    
    // On Submit 
    const onSubmit: SubmitHandler<LoginInput> = async ({email, password}) => { 
      if (login) { 
      console.log('You pressed the Sign in button');
      
      await sign_in(email, password)

    } else  {
      console.log('You pressed the Sign out button');      
      
      await sign_up(email, password)
      
    }
  };

  
  return (
    <div className='relative flex h-screen w-screen flex-col bg-black 
      md:justify-center md:bg-transparent md:items-center
    '>
      <Head>
        <title>Login Page</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Link href="/">
        <Image
          src="https://rb.gy/p2hphi"
          layout='fill'
          objectFit='cover'
          className='-z-10 !hidden opacity-70 sm:!inline'
        />
      </Link>

      <img 
        src="https://upload.wikimedia.org/wikipedia/commons/0/08/Netflix_2015_logo.svg" 
        alt="" 
        width={150}
        height={150}
        className="cursor-pointer object-contain absolute left-4 top-4
          md:left-10 md:top-6 duration-200 ease-in-out"
      />
{/* Login Forms  */}
      <form 
        onSubmit={handleSubmit(onSubmit)}
        action="" 
        className="relative mt-24 items-center grid rounded bg-black/75
          py-10 px-6 space-y-8 md:mt-0 md:max-w-md md:px-14 duration-[0.4s] ease-in-out"
      >
        <h1 className="text-4xl font-semibold">Sign in </h1>
        <div className='space-y-4'>
    {/* Email Input */}
          <label className="inline-block w-full ">
            <input 
              type="email" 
              placeholder='Email' 
              className='input'
              {...register('email', {required: true})}
            />
    {/* Errors will return when field validation fails */}
            {
              errors.email && 
              <p className="p-1 text-[13px] text-orange-400 font-normal">
                Please enter valid email address.
              </p>
            }
          </label>

    {/* Password Input */}
          <label className='inline-block w-full'>
            <input 
              type="password" 
              placeholder='Password' 
              className='input'
              {...register('password', {required: true })}            
            />
    {/* Errors will return when field validation fails */}
            {
              errors.email && 
                <p className="p-1 text-[13px] text-orange-400 font-normal">
                  Your password must contain between 4 and 60 characters.
                </p>
            }
          </label>

    {/* Remember Me? Need Help? */}
          <div className='flex items-center justify-between relative bottom-3'>
            <div className='inline-flex items-center p-1'>
              <input type="checkbox" className="placeholder-[gray] outline-none bg-[#333] focus:bg-[#454545]" />
              <p className="font-regular text-[13px] text-gray-500 p-1 ">Remember me</p>
            </div>
            <p className='p-1 text-[13px]'>Need Help?</p>
          </div>

        </div>
  {/* Sign In Button */}
        <button 
          type='submit'
          onClick={() => setlogin(true)}
          className="py-3 bg-[#e50914] rounded w-full font-semibold">
          Sign In
        </button>

  {/* Forgot Password .. */}
        <div className='text-gray-500'>
          New to Netflix? 
          <button 
            onClick={() => setlogin(false)}  
            type="submit" 
            className='relative left-1 text-white hover:underline'
          >
            Sign up now.{' '}
          </button>
        </div>
        
      </form>

    </div>
  )
}

export default Login

