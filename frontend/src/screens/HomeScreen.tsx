import { View, StyleSheet, Dimensions, Text } from 'react-native'
import React from 'react'
import SwiperFlatList from 'react-native-swiper-flatlist';
import EntityView from '../components/EntityView';
import { useCurrentUser } from '../slices/authSlice';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';

import type { StaticScreenProps } from '@react-navigation/native';

type RootStackParamList = {
  Home: undefined;
  Login: undefined;
  ProductList: { 
    type: string;
    title: string;
  };
};  

type HomeScreenNavigationProp = NativeStackNavigationProp<RootStackParamList, 'Home'>;

type ProductListProps = StaticScreenProps<{
  type: string;
  title: string;
}>;

export default function HomeScreen( { route }: ProductListProps ) {
  const navigation = useNavigation<HomeScreenNavigationProp>();
  const user = useCurrentUser();

  const entities = [
    {
      id: '1',
      imageUrl: 'https://www.uv.es/gammmm/Subsitio%20Operaciones/images/reactivos_solidos.jpg',
      title: 'Reactivo',
      onView: () => navigation.navigate('ProductList', { 
        type: 'Reactivo',
        title: 'Lista de Reactivos'
      }),
    },
    {
      id: '2',
      imageUrl: 'https://www.eldivinopastor.com/wp-content/uploads/Instrumentos-de-laboratorio-de-quimica.jpg',
      title: 'Material',
      onView: () => navigation.navigate('ProductList', {
        type: 'Material',
        title: 'Lista de Materiales'
      }),
    },
    {
      id: '3',
      imageUrl: 'https://ieqfb.com/wp-content/uploads/postgrado-en-operaciones-en-laboratorio.jpg',
      title: 'Auxiliar',
      onView: () => navigation.navigate('ProductList', {
        type: 'Auxiliar',
        title: 'Lista de Auxiliares'
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
                onEdit={() => console.log('Editar', item.id)}
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


