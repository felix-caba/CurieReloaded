import React, { useEffect } from 'react';
import type { PropsWithChildren } from 'react';
import { StyleSheet, Text, useColorScheme, View } from 'react-native';
import { Provider } from 'react-redux';
import { store, persistor } from './src/store/store';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { Toasts } from '@backpackapp-io/react-native-toast';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { createStaticNavigation, NavigationContainer, NavigatorScreenParams, StaticParamList, StaticScreenProps } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import HomeScreen from './src/screens/HomeScreen';
import LoginScreen from './src/screens/LoginScreen';
import { PersistGate } from 'redux-persist/integration/react';
import { useIsSignedIn } from './src/slices/authSlice';
import ProductListScreen from './src/screens/ProductListScreen';


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
    ProductList: {
      screen: ProductListScreen,
      options: {
        headerShown: true,
      },
    },
  },
});



const Navigation = createStaticNavigation(RootStack);

export type RootStackParamList = StaticParamList<typeof RootStack>;

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
