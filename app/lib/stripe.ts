import { createCheckoutSession, getStripePayments } from '@stripe/firestore-stripe-payments'
import { getFunctions, httpsCallable } from '@firebase/functions'
import app from '../firebase'


/* 
    Products: Access to High quality movies 
    Customers: Membership 
*/
const payments = getStripePayments(app, { 
    productsCollection: 'Membership',
    customersCollection: 'Customers'
})
/* Create a checkout */
export const load_checkout = async (price_id: string) => {
    await createCheckoutSession(payments, {
        price: price_id,
        //  Return to the original window 
        //  Can be changed to http://localhost
        success_url: window.location.origin,
        cancel_url: window.location.origin
    })
    .then((checkout_session) => window.location.assign(checkout_session.url))
    .catch((e) => console.log(e.message))
}

export const updateUserMembership = async () => {
    const instance = getFunctions(app, 'australia-southeast1');
    const function_ref = httpsCallable(instance, 'ext-firestore-stripe-payments-createPortalLink');

    await function_ref({
        return_url: `${window.location.origin}/account`
    }).then(({ data } : any) => window.location.assign(data.url)  )
    .catch((e) => console.log(e.message))
    
}



export default payments
