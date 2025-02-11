import { View, StyleSheet, Dimensions, Text } from 'react-native'
import React from 'react'
import SwiperFlatList from 'react-native-swiper-flatlist';
import EntityView from '../components/EntityView';
import { useCurrentUser } from '../slices/authSlice';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';

import type { StaticScreenProps } from '@react-navigation/native';
import { RootStackParamList } from '../../App';
import { useGetReactivosQuery } from '../services/productService';
import { Reactivo, Material, Auxiliar } from '../types/product';


export default function HomeScreen() {

  const { data, isError } = useGetReactivosQuery();
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const user = useCurrentUser();

  if (isError) {
    console.log('Error fetching reactivos:', isError);
  }
 
  const entities = [
    {
      id: '1',
      imageUrl: 'https://www.uv.es/gammmm/Subsitio%20Operaciones/images/reactivos_solidos.jpg',
      title: 'Reactivo', 
      onView: () => navigation.navigate('ProductsView', {
        products: data as Reactivo[],
      }),
    },
    {
      id: '2',
      imageUrl: 'https://www.eldivinopastor.com/wp-content/uploads/Instrumentos-de-laboratorio-de-quimica.jpg',
      title: 'Material',
      onView: () => navigation.navigate('ProductsView', {
        products: data as Material[],
      }),
    },
    {
      id: '3',
      imageUrl: 'https://ieqfb.com/wp-content/uploads/postgrado-en-operaciones-en-laboratorio.jpg',
      title: 'Auxiliar',
      onView: () => navigation.navigate('ProductsView', {
        products: data as Auxiliar[],
      }),
    },
  ];

  return (
    <View style={styles.container}>
      <Text style={styles.welcomeText}>Bienvenido, {user?.username}</Text>
      <View style={styles.swiperWrapper}>
        <SwiperFlatList
          data={entities}
          showPagination
          renderItem={({ item }) => (
            <View style={styles.child}>
              <EntityView
                imageUrl={item.imageUrl}
                title={item.title}
                onView={item.onView}
                onCreate={() => console.log('Crear nuevo')}
              />
            </View>
          )}
        />
      </View>
    </View>
  )
}

const { width } = Dimensions.get('window');

const styles = StyleSheet.create({
  container: { 
    flex: 1, 
    backgroundColor: 'white',
    paddingTop: 20,
  },
  swiperWrapper: {
    flexGrow: 0,
    paddingBottom: 40,
  },
  child: { 
    width,
    paddingHorizontal: 20,
  },
  welcomeText: {
    fontSize: 24,
    fontWeight: '600',
    marginBottom: 25,
    marginHorizontal: 20,
    color: '#2C3E50',
    letterSpacing: 0.5,
    backgroundColor: '#f2f2f2',
    padding: 15,
    borderRadius: 12,
  },
});


