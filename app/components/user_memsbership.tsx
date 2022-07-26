import CircularProgress from '@mui/material/CircularProgress';
import React, { useState } from 'react'
import useAuth from '../hooks/useAuth'
import useSubscription from '../hooks/useSubscription';
import { updateUserMembership } from '../lib/stripe';

function UserMembership() {
  const {user} = useAuth();
  const subscription = useSubscription(user);
  const [billLoading, setBillingLoading] = useState(false);

  const manageSubscription = () => { 
    if (subscription) { 
        setBillingLoading(true);
        updateUserMembership();
    }
  }

  return (
    <section>
        <div className='mt-4 grid grid-cols-1 gap-x-4 space-y-6 p-4  md:grid-cols-4 md:border-x-0 md:border-t
            md:border-b-0 md:px-0 md:pb-0'>
            <div>
                <h3 className="text-lg text-white">Membership & Billing</h3>
                <div className='space-y-5 py-7'>
                    <button 
                        onClick={manageSubscription}
                        disabled={!subscription || billLoading}
                        className={` h-10 w-[50%] rounded-sm bg-white text-black shadow-md
                            text-sm font-medium hover:bg-gray-300 whitespace-nowrap 
                            ${billLoading && 'opacity-60'}`}>
                        {
                            billLoading ? ( 
                                <CircularProgress color='inherit' size={20}/>
                            ): ( 
                                'Cancel Membership' 
                            )
                        }
                    </button>
                    <p className='text-sm text-gray-400'>
                        {
                            subscription?.cancel_at_period_end ? 
                                'Your membership will end on' : 
                                'Your next billing date is '
                        }
                        {
                            subscription?.current_period_end
                        }
                    </p>
                </div>
            </div> 
            <div className='items-center'>
                <ul className=''>
                    <div className='mx-auto mb-3'>
                        <p className='font-medium'>{user?.email}</p>
                        <p className='text-[gray]'>Password: ********* </p>
                    </div>
                    <li><p className='hover:underline text-blue-500 text-sm cursor-pointer'>Manage payment info</p></li>
                    <li><p className='hover:underline text-blue-500 text-sm cursor-pointer'>Add backup payment method</p></li>
                    <li><p className='hover:underline text-blue-500 text-sm cursor-pointer'>Billing details</p></li>
                    <li><p className='hover:underline text-blue-500 text-sm cursor-pointer'>Change billing day</p></li>
                </ul>
            </div>
            <div className='items-center text-right justify-end col-end-5 col-span-2'>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Change email</p>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Change password</p>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Add phone number</p>
            </div>
          </div>
    </section>
  )
}

export default UserMembership