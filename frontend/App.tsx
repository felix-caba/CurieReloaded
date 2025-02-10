import React, { useEffect } from 'react';
import type { PropsWithChildren } from 'react';
import { StyleSheet, Text, useColorScheme, View } from 'react-native';
import { Provider } from 'react-redux';
import { store, persistor } from './src/store/store';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { Toasts } from '@backpackapp-io/react-native-toast';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { createStaticNavigation, NavigationContainer, NavigatorScreenParams, StaticParamList, StaticScreenProps } 
from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import HomeScreen from './src/screens/HomeScreen';
import LoginScreen from './src/screens/LoginScreen';
import { PersistGate } from 'redux-persist/integration/react';
import { useIsSignedIn } from './src/slices/authSlice';
import ProductsView from './src/screens/ProductsView';
import { Auxiliar } from './src/types/product';
import { Material } from './src/types/product';
import { Reactivo } from './src/types/product';
import DetailScreen from './src/screens/DetailScreen';


const RootStack = createNativeStackNavigator({
  screens: {
    Login: {
      if: () => !useIsSignedIn(),
      screen: LoginScreen,
      options: {
        headerShown: false,
      },
    },
    Home: {
      if: () => useIsSignedIn(),
      screen: HomeScreen,
      options: {
        headerShown: false,
      },
    },
    ProductsView: {
      screen: ProductsView,
      options: {
        headerShown: true,
      },
    },
    DetailsScreen: {
      screen: DetailScreen,
      options: {
        headerShown: true,
      },
    },
  },
});

const Navigation = createStaticNavigation(RootStack);

export type RootStackParamList = {
  Login: undefined;
  Home: undefined;
  ProductsView: {
    products: (Reactivo | Material | Auxiliar)[];
  };
  DetailsScreen: {
    product: Reactivo | Material | Auxiliar;
  }
};

declare global {
  namespace ReactNavigation {
    interface RootParamList extends RootStackParamList { }
  }
}

function App(): React.JSX.Element {

  return (
    <SafeAreaProvider>
      <GestureHandlerRootView>




        <Provider store={store}>
          <PersistGate loading={null} persistor={persistor}>


            <Navigation />



            <Toasts />
          </PersistGate>
        </Provider>


      </GestureHandlerRootView>
    </SafeAreaProvider>


  );
}





const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    padding: 20,
  },
});


export default App;
