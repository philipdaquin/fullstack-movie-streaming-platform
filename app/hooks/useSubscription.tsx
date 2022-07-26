import { onCurrentUserSubscriptionUpdate, Subscription } from "@stripe/firestore-stripe-payments";
import { User } from "firebase/auth";
import { useState, useEffect } from "react";
import payments from "../lib/stripe";
function useSubscription(user: User | null) {
    const [subscribe, setSubscribe] = useState<Subscription | null>(null);

    useEffect(() => { 
        if (!user) return 

        onCurrentUserSubscriptionUpdate(payments, (snapshot) => { 
            setSubscribe(snapshot.subscriptions
                .filter((subscription) => 
                subscription.status === "active" || 
                subscription.status === "trialing")[0])
        })

    }, [user])
    //  Return the list of subscriptions
    return subscribe
}

export default useSubscription