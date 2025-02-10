import React, { useState, useMemo } from 'react';
import { View, Text, FlatList, TextInput, TouchableOpacity, StyleSheet } from 'react-native';
import { StaticScreenProps, useNavigation } from '@react-navigation/native';
import { Reactivo, Material, Auxiliar } from '../types/product';
import { NativeStackNavigationProp, NativeStackScreenProps } from '@react-navigation/native-stack';
import { RootStackParamList } from '../../App';

type Props = NativeStackScreenProps<RootStackParamList, 'ProductsView'>;

export default function ProductsView({ route }: Props) {

  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  
  const { products } = route.params;
  const [searchQuery, setSearchQuery] = useState('');
  const [sortByCantidad, setSortByCantidad] = useState(false);

  const filteredProducts = useMemo(() => {
    return products.filter(product =>
      product?.nombre?.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }, [products, searchQuery]);

    const sortedProducts = useMemo(() => {
        if (sortByCantidad) {
            return [...filteredProducts].sort((a, b) => (a.cantidad ?? 0) - (b.cantidad ?? 0));
        }
    return filteredProducts;
  }, [filteredProducts, sortByCantidad]);

  const renderProductItem = ({ item }: { item: Reactivo | Material | Auxiliar }) => (
    <TouchableOpacity style={styles.productItem} onPress={() => navigation.navigate('DetailsScreen', { product: item })}>
      <Text style={styles.productName}>{item.nombre}</Text>
      <Text>Cantidad: {item.cantidad ?? 'N/A'}</Text>
    </TouchableOpacity>
  );

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Productos Overview</Text>

      <TextInput
        style={styles.searchInput}
        placeholder="Buscar producto..."
        value={searchQuery}
        onChangeText={setSearchQuery}
      />
        <TouchableOpacity
            style={styles.sortButton}
            onPress={() => setSortByCantidad(!sortByCantidad)}
        >
            <Text style={styles.sortButtonText}>
                Ordenar por Cantidad ({sortByCantidad ? 'Ascendente' : 'Descendente'})
            </Text>
      </TouchableOpacity>

      <FlatList
        data={sortedProducts}
        renderItem={renderProductItem}
        keyExtractor={(item, index) => String(item.idProducto ?? index)}
        numColumns={2}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 10,
  },
  title: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 10,
  },
  searchInput: {
    height: 40,
    borderColor: 'gray',
    borderWidth: 1,
    marginBottom: 10,
    paddingHorizontal: 10,
  },
  productItem: {
    flex: 1,
    margin: 5,
    padding: 10,
    borderWidth: 1,
    borderColor: '#ddd',
    borderRadius: 5,
    alignItems: 'center',
  },
  productName: {
    fontWeight: 'bold',
  },
    sortButton: {
        backgroundColor: '#007bff',
        padding: 10,
        borderRadius: 5,
        marginBottom: 10,
    },
    sortButtonText: {
        color: '#fff',
        textAlign: 'center',
    },
});